use crate::ast::*;
use crate::lexer::Token;
use std::iter::Peekable;
use std::slice::Iter;

pub fn parse(tokens: &[Token]) -> Result<Vec<Stmt>, String> {
    let mut iter = tokens.iter().peekable();
    let mut stmts = Vec::new();
    
    while iter.peek().is_some() {
        stmts.push(parse_stmt(&mut iter)?);
    }
    Ok(stmts)
}

fn parse_stmt(iter: &mut Peekable<Iter<Token>>) -> Result<Stmt, String> {
    match iter.peek() {
        Some(Token::Keyword(k)) if k == "material" => {
            iter.next(); // consume 'material'
            let name = match iter.next() {
                Some(Token::Ident(n)) => n.clone(),
                _ => return Err("Expected material name".into()),
            };
            
            let mut parent = None;
            if let Some(Token::Keyword(k)) = iter.peek() {
                if k == "extends" {
                    iter.next();
                    parent = match iter.next() {
                        Some(Token::Ident(n)) => Some(n.clone()),
                        _ => return Err("Expected parent material name".into()),
                    };
                }
            }
            
            expect_token(iter, &Token::LBrace)?;
            let mut properties = Vec::new();
            while iter.peek() != Some(&&Token::RBrace) {
                let key = match iter.next() {
                    Some(Token::Ident(k)) => k.clone(),
                    _ => return Err("Expected property name in material".into()),
                };
                expect_token(iter, &Token::Colon)?;
                let val = parse_expr(iter, 0)?;
                properties.push((key, val));
            }
            expect_token(iter, &Token::RBrace)?;
            
            Ok(Stmt::Material { name, parent, properties })
        }
        Some(Token::Keyword(k)) if k == "let" => {
            iter.next();
            let name = match iter.next() {
                Some(Token::Ident(n)) => n.clone(),
                _ => return Err("Expected variable name".into()),
            };
            expect_token(iter, &Token::Equal)?;
            let value = parse_expr(iter, 0)?;
            Ok(Stmt::Let { name, value })
        }
        Some(Token::Keyword(k)) if k == "scene" => {
            iter.next();
            expect_token(iter, &Token::LBrace)?;
            let mut items = Vec::new();
            while iter.peek() != Some(&&Token::RBrace) {
                let item = match iter.next() {
                    Some(Token::Keyword(k)) if k == "camera" => {
                        expect_token(iter, &Token::LBrace)?;
                        let mut properties = Vec::new();
                        while iter.peek() != Some(&&Token::RBrace) {
                            let key = match iter.next() {
                                Some(Token::Ident(k)) => k.clone(),
                                _ => return Err("Expected property name in camera".into()),
                            };
                            expect_token(iter, &Token::Colon)?;
                            let val = parse_expr(iter, 0)?;
                            properties.push((key, val));
                            if iter.peek() == Some(&&Token::Comma) { iter.next(); }
                        }
                        expect_token(iter, &Token::RBrace)?;
                        SceneItem::Camera { properties }
                    }
                    Some(Token::Keyword(k)) if k == "light" => {
                        expect_token(iter, &Token::LBrace)?;
                        let mut properties = Vec::new();
                        while iter.peek() != Some(&&Token::RBrace) {
                            let key = match iter.next() {
                                Some(Token::Ident(k)) => k.clone(),
                                _ => return Err("Expected property name in light".into()),
                            };
                            expect_token(iter, &Token::Colon)?;
                            let val = parse_expr(iter, 0)?;
                            properties.push((key, val));
                            if iter.peek() == Some(&&Token::Comma) { iter.next(); }
                        }
                        expect_token(iter, &Token::RBrace)?;
                        SceneItem::Light { properties }
                    }
                    Some(Token::Keyword(k)) if k == "object" => {
                        expect_token(iter, &Token::LParen)?;
                        let expr = parse_expr(iter, 0)?;
                        let mut properties = Vec::new();
                        while iter.peek() == Some(&&Token::Comma) {
                            iter.next();
                            let mut key = None;
                            if let Some(Token::Ident(k)) = iter.peek() {
                                key = Some(k.clone());
                            } else if let Some(Token::Keyword(k)) = iter.peek() {
                                key = Some(k.clone());
                            }
                            if let Some(k) = key {
                                iter.next(); // consume
                                expect_token(iter, &Token::Colon)?;
                                let val = parse_expr(iter, 0)?;
                                properties.push((k, val));
                            } else {
                                break;
                            }
                        }
                        expect_token(iter, &Token::RParen)?;
                        SceneItem::Object { expr, properties }
                    }
                    _ => return Err("Expected camera, light, or object in scene".into()),
                };
                items.push(item);
            }
            expect_token(iter, &Token::RBrace)?;
            Ok(Stmt::Scene { items })
        }
        _ => Err(format!("Unexpected token at top level: {:?}", iter.peek())),
    }
}

fn expect_token(iter: &mut Peekable<Iter<Token>>, expected: &Token) -> Result<(), String> {
    let next = iter.next();
    if next == Some(expected) {
        Ok(())
    } else {
        Err(format!("Expected {:?}, got {:?}", expected, next))
    }
}

fn infix_binding_power(op: &Token) -> Option<(u8, u8)> {
    match op {
        Token::Pipe => Some((1, 2)), // union, lowest
        Token::Minus => Some((3, 4)), // diff
        Token::Ampersand => Some((5, 6)), // intersect
        Token::Star => Some((7, 8)), // mul, highest
        _ => None,
    }
}

fn parse_expr(iter: &mut Peekable<Iter<Token>>, min_bp: u8) -> Result<Expr, String> {
    let mut lhs = match iter.next() {
        Some(Token::Number(n)) => Expr::Number(*n),
        Some(Token::String(s)) => Expr::String(s.clone()),
        Some(Token::Angle(a)) => Expr::Angle(*a),
        Some(Token::Ident(id)) => {
            if iter.peek() == Some(&&Token::LParen) {
                // function call
                iter.next(); // consume (
                let mut positional = Vec::new();
                let mut named = Vec::new();
                
                while iter.peek() != Some(&&Token::RParen) {
                    if let Some(Token::Ident(k)) = iter.peek().cloned() {
                        let mut cloned_iter = iter.clone();
                        cloned_iter.next(); // skip ident
                        if cloned_iter.peek() == Some(&&Token::Colon) {
                            // named argument
                            iter.next(); // consume ident
                            iter.next(); // consume colon
                            let val = parse_expr(iter, 0)?;
                            named.push((k.clone(), val));
                            if iter.peek() == Some(&&Token::Comma) { iter.next(); }
                            continue;
                        }
                    }
                    // positional
                    let val = parse_expr(iter, 0)?;
                    positional.push(val);
                    if iter.peek() == Some(&&Token::Comma) { iter.next(); }
                }
                expect_token(iter, &Token::RParen)?;
                Expr::Call { callee: id.clone(), positional, named }
            } else {
                Expr::Ident(id.clone())
            }
        },
        Some(Token::LBracket) => {
            let mut vec = Vec::new();
            while iter.peek() != Some(&&Token::RBracket) {
                vec.push(parse_expr(iter, 0)?);
                if iter.peek() == Some(&&Token::Comma) { iter.next(); }
            }
            expect_token(iter, &Token::RBracket)?;
            Expr::Vector(vec)
        }
        Some(Token::Keyword(k)) if k == "for" => {
            let var = match iter.next() {
                Some(Token::Ident(v)) => v.clone(),
                _ => return Err("Expected loop variable".into()),
            };
            expect_token(iter, &Token::Keyword("in".into()))?;
            let start = match iter.next() {
                Some(Token::Number(n)) => *n as i32,
                _ => return Err("Expected start number".into()),
            };
            expect_token(iter, &Token::DotDot)?;
            let end = match iter.next() {
                Some(Token::Number(n)) => *n as i32,
                _ => return Err("Expected end number".into()),
            };
            expect_token(iter, &Token::LBrace)?;
            let body = parse_expr(iter, 0)?;
            expect_token(iter, &Token::RBrace)?;
            Expr::For { var, start, end, body: Box::new(body) }
        }
        Some(Token::LParen) => {
            let expr = parse_expr(iter, 0)?;
            expect_token(iter, &Token::RParen)?;
            expr
        }
        other => return Err(format!("Unexpected token in expression: {:?}", other)),
    };
    
    loop {
        let op = match iter.peek() {
            None => break,
            Some(op) => *op,
        };
        
        if let Some((l_bp, r_bp)) = infix_binding_power(op) {
            if l_bp < min_bp {
                break;
            }
            let op = iter.next().unwrap().clone();
            
            let bin_op = match op {
                Token::Pipe => BinOp::Union,
                Token::Minus => BinOp::Difference,
                Token::Ampersand => BinOp::Intersect,
                Token::Star => BinOp::Mul,
                _ => unreachable!(),
            };
            
            let rhs = parse_expr(iter, r_bp)?;
            lhs = Expr::Binary(bin_op, Box::new(lhs), Box::new(rhs));
            continue;
        }
        break;
    }
    
    Ok(lhs)
}
