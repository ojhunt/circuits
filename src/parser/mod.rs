mod ast;
mod circuit_parser;
mod lexer;
mod parser;

#[cfg(test)]
mod test;

pub fn parse(test: &str) {
    let _ = circuit_parser::ModuleParser::new().parse(test);
}
