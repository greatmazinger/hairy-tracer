#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    String(String),
    Number(f64),
    Angle(f64), // degrees in source
    Vector(Vec<Expr>),
    Ident(String),
    Call {
        callee: String,
        positional: Vec<Expr>,
        named: Vec<(String, Expr)>,
    },
    Binary(BinOp, Box<Expr>, Box<Expr>),
    For {
        var: String,
        start: i32,
        end: i32,
        body: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Mul,
    Union,
    Intersect,
    Difference,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Material {
        name: String,
        parent: Option<String>,
        properties: Vec<(String, Expr)>,
    },
    Let {
        name: String,
        value: Expr,
    },
    Scene {
        items: Vec<SceneItem>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SceneItem {
    Camera { properties: Vec<(String, Expr)> },
    Light { properties: Vec<(String, Expr)> },
    Object { expr: Expr, properties: Vec<(String, Expr)> },
    EnvMap(String),
}
