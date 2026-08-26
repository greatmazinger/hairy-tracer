use crate::ast::*;
use serde_json::{json, Value, Map};
use std::collections::HashMap;

pub fn elaborate(stmts: &[Stmt]) -> Result<Value, String> {
    let mut env: HashMap<String, Value> = HashMap::new();
    let mut materials = Map::new();
    let mut scene_out = Map::new();
    
    // Add vector built-ins
    env.insert("x".to_string(), json!([1.0, 0.0, 0.0]));
    env.insert("y".to_string(), json!([0.0, 1.0, 0.0]));
    env.insert("z".to_string(), json!([0.0, 0.0, 1.0]));

    for stmt in stmts {
        match stmt {
            Stmt::Material { name, parent, properties } => {
                let mut mat = Map::new();
                if let Some(p) = parent {
                    if let Some(Value::Object(p_mat)) = env.get(p) {
                        for (k, v) in p_mat {
                            mat.insert(k.clone(), v.clone());
                        }
                    }
                }
                for (k, v) in properties {
                    mat.insert(k.clone(), eval_expr(v, &env)?);
                }
                materials.insert(name.clone(), Value::Object(mat.clone()));
                env.insert(name.clone(), Value::Object(mat)); // store mat ref
            }
            Stmt::Let { name, value } => {
                let val = eval_expr(value, &env)?;
                env.insert(name.clone(), val);
            }
            Stmt::Scene { items } => {
                let mut objects = Vec::new();
                for item in items {
                    match item {
                        SceneItem::Camera { properties } => {
                            let mut cam = Map::new();
                            for (k, v) in properties {
                                cam.insert(k.clone(), eval_expr(v, &env)?);
                            }
                            scene_out.insert("camera".to_string(), Value::Object(cam));
                        }
                        SceneItem::EnvMap(path) => { scene_out.insert("environment_map".to_string(), serde_json::Value::String(path.clone())); }
                        SceneItem::Light { properties } => {
                            let mut lights = match scene_out.get_mut("lights") {
                                Some(Value::Array(arr)) => arr,
                                _ => {
                                    scene_out.insert("lights".to_string(), Value::Array(Vec::new()));
                                    scene_out.get_mut("lights").unwrap().as_array_mut().unwrap()
                                }
                            };
                            let mut light = Map::new();
                            for (k, v) in properties {
                                light.insert(k.clone(), eval_expr(v, &env)?);
                            }
                            lights.push(Value::Object(light));
                        }
                        SceneItem::Object { expr, properties } => {
                            let mut obj = eval_expr(expr, &env)?;
                            
                            // Attach properties (like material) to the root of the obj if needed
                            if let Value::Object(ref mut map) = obj {
                                for (k, v) in properties {
                                    if k == "material" {
                                        if let Expr::Ident(ref m_name) = v {
                                            map.insert("material".to_string(), Value::String(m_name.clone()));
                                        }
                                    } else {
                                        map.insert(k.clone(), eval_expr(v, &env)?);
                                    }
                                }
                            }
                            objects.push(obj);
                        }
                    }
                }
                scene_out.insert("objects".to_string(), Value::Array(objects));
            }
        }
    }
    
    scene_out.insert("materials".to_string(), Value::Object(materials));
    if !scene_out.contains_key("integrator") {
        scene_out.insert("integrator".to_string(), Value::String("pathtracer".to_string()));
    }
    
    Ok(Value::Object(scene_out))
}

fn eval_expr(expr: &Expr, env: &HashMap<String, Value>) -> Result<Value, String> {
    match expr {
        Expr::Number(n) => {
            if n.fract() == 0.0 {
                Ok(json!(*n as i64))
            } else {
                Ok(json!(n))
            }
        },
        Expr::String(s) => Ok(json!(s)),
        Expr::Angle(a) => Ok(json!(a)), // Angle erased to f64? Wait, if we keep degrees, no problem. Wait! "Angle erased to bare radian f64". But fov_degrees needs degrees! Let's just output the f64. If it's used as radians in rotate, we might need to convert. Wait, the engine schema rotate uses degrees.
        Expr::Vector(v) => {
            let mut arr = Vec::new();
            for e in v {
                arr.push(eval_expr(e, env)?);
            }
            Ok(Value::Array(arr))
        }
        Expr::Ident(id) => {
            if let Some(v) = env.get(id) {
                Ok(v.clone())
            } else if id == "x" {
                Ok(json!("x"))
            } else if id == "y" {
                Ok(json!("y"))
            } else if id == "true" { Ok(serde_json::Value::Bool(true)) } else if id == "false" { Ok(serde_json::Value::Bool(false)) } else if id == "z" {
                Ok(json!("z"))
            } else {
                Err(format!("Undefined variable in eval: {}", id))
            }
        }
        Expr::Binary(op, lhs, rhs) => {
            let l_val = eval_expr(lhs, env)?;
            let r_val = eval_expr(rhs, env)?;
            
            match op {
                BinOp::Mul => {
                    let l = l_val.as_f64().unwrap();
                    let r = r_val.as_f64().unwrap();
                    Ok(json!(l * r))
                }
                BinOp::Union | BinOp::Difference | BinOp::Intersect => {
                    let op_str = match op {
                        BinOp::Union => "union",
                        BinOp::Difference => "difference",
                        BinOp::Intersect => "intersection",
                        _ => unreachable!(),
                    };
                    Ok(json!({
                        "type": "csg",
                        "op": op_str,
                        "left": l_val,
                        "right": r_val
                    }))
                }
            }
        }
        Expr::For { var, start, end, body } => {
            let mut union_tree = None;
            for i in *start..*end {
                let mut new_env = env.clone();
                new_env.insert(var.clone(), json!(i as f64));
                
                let iter_val = eval_expr(body, &new_env)?;
                if let Some(tree) = union_tree {
                    union_tree = Some(json!({
                        "type": "csg",
                        "op": "union",
                        "left": tree,
                        "right": iter_val
                    }));
                } else {
                    union_tree = Some(iter_val);
                }
            }
            Ok(union_tree.unwrap_or(json!({}))) // empty CSG? fallback if loop 0 times
        }
        Expr::Call { callee, positional, named } => {
            let mut map = Map::new();
            match callee.as_str() {
                "cube" | "cylinder" | "sphere" | "plane" => {
                    map.insert("type".to_string(), Value::String(callee.clone()));
                    for (k, v) in named {
                        map.insert(k.clone(), eval_expr(v, env)?);
                    }
                }
                "translate" | "rotate" | "scale" => {
                    map.insert("type".to_string(), Value::String("transform".to_string()));
                    let child = eval_expr(&positional[0], env)?;
                    map.insert("child".to_string(), child);
                    
                    let mut vec = vec![0.0, 0.0, 0.0];
                    for (k, v) in named {
                        let val = eval_expr(v, env)?.as_f64().unwrap();
                        if k == "x" { vec[0] = val; }
                        else if k == "y" { vec[1] = val; }
                        else if k == "z" { vec[2] = val; }
                    }
                    map.insert(callee.clone(), json!(vec));
                }
                _ => return Err(format!("Unknown function in eval: {}", callee)),
            }
            Ok(Value::Object(map))
        }
    }
}
