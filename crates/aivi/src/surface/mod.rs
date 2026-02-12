mod ast;
mod parser;

pub use ast::*;
pub use parser::{parse_modules, parse_modules_from_tokens};

#[cfg(test)]
mod tests;
