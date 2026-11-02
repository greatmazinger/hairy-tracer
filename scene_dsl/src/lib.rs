pub mod lexer;
pub mod ast;
pub mod parser;
pub mod typecheck;
pub mod elaborator;
pub mod emitter;

/// Compiles a Scene DSL source string into a JSON scene configuration string.
pub fn compile_dsl(source: &str) -> Result<String, String> {
    let tokens = lexer::lex(source).map_err(|e| format!("Lexer error: {e:?}"))?;
    let ast = parser::parse(&tokens).map_err(|e| format!("Parser error: {e:?}"))?;
    typecheck::typecheck(&ast).map_err(|e| format!("Typecheck error: {e:?}"))?;
    let ir = elaborator::elaborate(&ast).map_err(|e| format!("Elaboration error: {e:?}"))?;
    Ok(emitter::emit(&ir))
}
