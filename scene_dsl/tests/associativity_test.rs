use scene_dsl::{lexer, parser, ast};

#[test]
fn test_difference_associativity() {
    let input = "let x = a - b - c";
    let tokens = lexer::lex(input).unwrap();
    let stmts = parser::parse(&tokens).unwrap();
    
    // Should be (a - b) - c
    if let ast::Stmt::Let { value, .. } = &stmts[0] {
        if let ast::Expr::Binary(ast::BinOp::Difference, lhs, rhs) = value {
            // rhs should be c
            if let ast::Expr::Ident(ref name) = **rhs {
                assert_eq!(name, "c");
            } else {
                panic!("Right side of root should be 'c', got {:?}", rhs);
            }
            
            // lhs should be (a - b)
            if let ast::Expr::Binary(ast::BinOp::Difference, ref llhs, ref lrhs) = **lhs {
                if let ast::Expr::Ident(ref lname) = **llhs {
                    assert_eq!(lname, "a");
                } else {
                    panic!("Left side of inner should be 'a'");
                }
                
                if let ast::Expr::Ident(ref rname) = **lrhs {
                    assert_eq!(rname, "b");
                } else {
                    panic!("Right side of inner should be 'b'");
                }
            } else {
                panic!("Left side of root should be a difference (a - b), got {:?}", lhs);
            }
        } else {
            panic!("Expected root to be a difference, got {:?}", value);
        }
    } else {
        panic!("Expected Let statement");
    }
}
