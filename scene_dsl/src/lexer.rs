use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    String(String),
    Ident(String),
    Number(f64),
    Angle(f64),
    Keyword(String),
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Colon,
    Equal,
    Pipe,
    Ampersand,
    Minus,
    Star,
    DotDot,
}

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    
    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            '{' => { tokens.push(Token::LBrace); chars.next(); }
            '}' => { tokens.push(Token::RBrace); chars.next(); }
            '(' => { tokens.push(Token::LParen); chars.next(); }
            ')' => { tokens.push(Token::RParen); chars.next(); }
            '[' => { tokens.push(Token::LBracket); chars.next(); }
            ']' => { tokens.push(Token::RBracket); chars.next(); }
            ',' => { tokens.push(Token::Comma); chars.next(); }
            ':' => { tokens.push(Token::Colon); chars.next(); }
            '=' => { tokens.push(Token::Equal); chars.next(); }
            '|' => { tokens.push(Token::Pipe); chars.next(); }
            '&' => { tokens.push(Token::Ampersand); chars.next(); }
            '-' => { 
                chars.next();
                if let Some(&c2) = chars.peek() {
                    if c2.is_ascii_digit() {
                        let num = consume_number_or_angle(&mut chars, true)?;
                        tokens.push(num);
                        continue;
                    }
                }
                tokens.push(Token::Minus); 
            }
            '*' => { tokens.push(Token::Star); chars.next(); }
            '.' => {
                chars.next();
                if chars.next() == Some('.') {
                    tokens.push(Token::DotDot);
                } else {
                    return Err("Expected '..'".into());
                }
            }
                        '"' => {
                let mut s = String::new();
                chars.next();
                while let Some(&ch) = chars.peek() {
                    if ch == '"' {
                        chars.next();
                        break;
                    } else {
                        s.push(chars.next().unwrap());
                    }
                }
                tokens.push(Token::String(s));
            }
            '0'..='9' => {
                tokens.push(consume_number_or_angle(&mut chars, false)?);
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        ident.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                match ident.as_str() {
                    "let" | "for" | "in" | "material" | "extends" | "scene" | "camera" | "light" | "object" => {
                        tokens.push(Token::Keyword(ident));
                    }
                    _ => tokens.push(Token::Ident(ident)),
                }
            }
            _ => return Err(format!("Unexpected character: {}", c)),
        }
    }
    
    Ok(tokens)
}

fn consume_number_or_angle(chars: &mut Peekable<Chars>, negative: bool) -> Result<Token, String> {
    let mut num_str = String::new();
    if negative {
        num_str.push('-');
    }
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            num_str.push(chars.next().unwrap());
        } else if c == '.' {
            // Check if next is also '.'
            let mut clone = chars.clone();
            clone.next(); // skip current '.'
            if let Some(&'.') = clone.peek() {
                break; // it's a '..' token, don't consume '.'
            } else {
                num_str.push(chars.next().unwrap());
            }
        } else {
            break;
        }
    }
    let val: f64 = num_str.parse().map_err(|_| format!("Invalid number: {}", num_str))?;
    
    // check for deg
    let mut is_angle = false;
    let mut peek_idx = 0;
    
    let mut deg_chars = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_alphabetic() {
            deg_chars.push(chars.next().unwrap());
        } else {
            break;
        }
    }
    
    if deg_chars == "deg" {
        Ok(Token::Angle(val))
    } else if deg_chars.is_empty() {
        Ok(Token::Number(val))
    } else {
        Err(format!("Invalid suffix: {}", deg_chars))
    }
}
