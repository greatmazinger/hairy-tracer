use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    String,
    Scalar,
    Angle,
    Vector,
    Csg,
    Material,
}

pub fn typecheck(stmts: &[Stmt]) -> Result<(), String> {
    let mut env = HashMap::new();
    
    // Built-in functions
    env.insert("cube".to_string(), Type::Csg);
    env.insert("cylinder".to_string(), Type::Csg);
    env.insert("sphere".to_string(), Type::Csg);
    env.insert("plane".to_string(), Type::Csg);
    env.insert("translate".to_string(), Type::Csg);
    env.insert("rotate".to_string(), Type::Csg);
    env.insert("scale".to_string(), Type::Csg);
    env.insert("x".to_string(), Type::Vector);
    env.insert("y".to_string(), Type::Vector);
    env.insert("z".to_string(), Type::Vector);

    for stmt in stmts {
        match stmt {
            Stmt::Material { name, parent, properties } => {
                if let Some(p) = parent {
                    if env.get(p) != Some(&Type::Material) {
                        return Err(format!("Parent material {} not found", p));
                    }
                }
                env.insert(name.clone(), Type::Material);
            }
            Stmt::Let { name, value } => {
                let ty = typecheck_expr(value, &mut env)?;
                env.insert(name.clone(), ty);
            }
            Stmt::Scene { items } => {
                for item in items {
                    match item {
                        SceneItem::Camera { properties } => {
                            for (_, val) in properties {
                                typecheck_expr(val, &mut env)?;
                            }
                        }
                        SceneItem::EnvMap(_) => {},
                        SceneItem::Light { properties } => {
                            for (_, val) in properties {
                                typecheck_expr(val, &mut env)?;
                            }
                        }
                        SceneItem::Object { expr, properties } => {
                            let ty = typecheck_expr(expr, &mut env)?;
                            if ty != Type::Csg {
                                return Err("Object must be a CSG expression".into());
                            }
                            for (key, val) in properties {
                                let t = typecheck_expr(val, &mut env)?;
                                if key == "material" && t != Type::Material {
                                    return Err("Object material must be a Material".into());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn typecheck_expr(expr: &Expr, env: &mut HashMap<String, Type>) -> Result<Type, String> {
    match expr {
        Expr::Number(_) => Ok(Type::Scalar),
        Expr::String(_) => Ok(Type::String),
        Expr::Angle(_) => Ok(Type::Angle),
        Expr::Vector(v) => {
            for e in v {
                let ty = typecheck_expr(e, env)?;
                if ty != Type::Scalar {
                    return Err("Vector components must be scalars".into());
                }
            }
            Ok(Type::Vector)
        }
        Expr::Ident(id) => {
            if let Some(ty) = env.get(id) {
                Ok(ty.clone())
            } else {
                Err(format!("Undefined variable: {}", id))
            }
        }
        Expr::Call { callee, positional, named } => {
            if !env.contains_key(callee) {
                return Err(format!("Undefined function: {}", callee));
            }
            for e in positional {
                typecheck_expr(e, env)?;
            }
            for (_, e) in named {
                typecheck_expr(e, env)?;
            }
            Ok(Type::Csg) // all builtins return CSG
        }
        Expr::Binary(op, lhs, rhs) => {
            let l_ty = typecheck_expr(lhs, env)?;
            let r_ty = typecheck_expr(rhs, env)?;
            
            match op {
                BinOp::Mul => {
                    if (l_ty == Type::Scalar && r_ty == Type::Angle) || (l_ty == Type::Angle && r_ty == Type::Scalar) {
                        Ok(Type::Angle)
                    } else if l_ty == Type::Scalar && r_ty == Type::Scalar {
                        Ok(Type::Scalar)
                    } else {
                        Err(format!("Invalid multiplication between {:?} and {:?}", l_ty, r_ty))
                    }
                }
                BinOp::Union | BinOp::Difference | BinOp::Intersect => {
                    if l_ty == Type::Csg && r_ty == Type::Csg {
                        Ok(Type::Csg)
                    } else {
                        Err(format!("CSG operations require CSG types, got {:?} and {:?}", l_ty, r_ty))
                    }
                }
            }
        }
        Expr::For { var, start: _, end: _, body } => {
            env.insert(var.clone(), Type::Scalar);
            let b_ty = typecheck_expr(body, env)?;
            env.remove(var);
            if b_ty != Type::Csg {
                return Err("For loop body must be a CSG expression".into());
            }
            Ok(Type::Csg)
        }
    }
}
