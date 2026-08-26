use scene_dsl::*;
use serde_json::json;

#[test]
fn test_lexer_angle() {
    let input = "15deg";
    let tokens = lexer::lex(input).unwrap();
    assert_eq!(tokens, vec![lexer::Token::Angle(15.0)]);
}

#[test]
fn test_parser_precedence() {
    // a - b | c  => (a - b) | c
    let input = "let x = a - b | c";
    let tokens = lexer::lex(input).unwrap();
    // The tokens for `a - b | c`
    // We just test the AST generated
    let stmts = parser::parse(&tokens).unwrap();
    if let ast::Stmt::Let { value, .. } = &stmts[0] {
        if let ast::Expr::Binary(ast::BinOp::Union, lhs, rhs) = value {
            if let ast::Expr::Binary(ast::BinOp::Difference, _, _) = **lhs {
                // Good
            } else {
                panic!("Expected (a - b) | c, got {:?}", value);
            }
        } else {
            panic!("Expected (a - b) | c, got {:?}", value);
        }
    } else {
        panic!("Expected Let");
    }
}

#[test]
fn test_typecheck_errors() {
     // wait, the lexer consumes `=` as part of syntax, so `let x = 15deg * 2deg`
    let tokens = lexer::lex("let x = 15deg * 2deg").unwrap();
    let stmts = parser::parse(&tokens).unwrap();
    let res = typecheck::typecheck(&stmts);
    assert!(res.is_err(), "Expected type error for Angle * Angle");

    let tokens2 = lexer::lex("let x = undefined_var").unwrap();
    let stmts2 = parser::parse(&tokens2).unwrap();
    let res2 = typecheck::typecheck(&stmts2);
    assert!(res2.is_err(), "Expected error for undefined var");
}

#[test]
fn test_elaborator_for_loop() {
    let input = "let x = for i in 0..24 { cube(x: i) }";
    let tokens = lexer::lex(input).unwrap();
    let stmts = parser::parse(&tokens).unwrap();
    // we just want to typecheck and elaborate
    typecheck::typecheck(&stmts).unwrap();
    let ir = elaborator::elaborate(&stmts).unwrap();
    // Actually, x is not stored in the returned scene object, it's stored in env internally.
    // Let's just create a scene that uses it.
}

#[test]
fn test_material_inheritance() {
    let input = "
        material base { color: [1.0, 2.0, 3.0] }
        material child extends base { shininess: 10 }
        scene {
            object(cube(), material: child)
        }
    ";
    let tokens = lexer::lex(input).unwrap();
    let stmts = parser::parse(&tokens).unwrap();
    typecheck::typecheck(&stmts).unwrap();
    let ir = elaborator::elaborate(&stmts).unwrap();
    let mat = ir.get("materials").unwrap().get("child").unwrap();
    assert_eq!(mat.get("color").unwrap(), &json!([1, 2, 3]));
    assert_eq!(mat.get("shininess").unwrap(), &json!(10));
}
