pub mod eval;
pub mod parser;
pub mod token;

pub use eval::{clean_float, evaluate, format_result, AngleMode, EvalContext};
pub use parser::{Expr, Parser};
pub use token::Lexer;

/// Parses and calculates a mathematical expression given an evaluation context.
pub fn calculate(input: &str, ctx: &EvalContext) -> Result<f64, String> {
    let lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let expr = parser.parse()?;
    evaluate(&expr, ctx)
}

