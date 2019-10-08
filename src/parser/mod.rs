pub mod ast;
mod circuit_parser;
mod lexer;
mod parser;

#[cfg(test)]
mod test;

#[derive(Debug)]
struct Location {
    line: usize,
    column: usize,
}
#[derive(Debug)]
struct ParseError {
    error: String,
    location: Option<Location>,
}

impl std::fmt::Display for ParseError {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        return Ok(());
    }
}

impl std::error::Error for ParseError {}

pub fn parse<'a>(test: &str) -> Result<ast::Module, Box<dyn std::error::Error + 'static>> {
    let result = circuit_parser::ModuleParser::new().parse(test);
    return Ok(result.map_err(|error| {
        let location = match error.clone() {
            lalrpop_util::ParseError::InvalidToken { location } => Some(location),
            lalrpop_util::ParseError::UnrecognizedEOF {
                location,
                expected: _,
            } => Some(location),
            lalrpop_util::ParseError::UnrecognizedToken {
                token: (location, _, _),
                expected: _,
            } => {
                println!("HERE: {:?}", location);
                Some(location)
            }
            _ => None,
        };
        let location = location.map(|location| {
            let indices = test.char_indices().take(location);
            let lines = indices.filter(|(_, ch)| return *ch == '\n');
            let line = 1 + lines.clone().count();
            let (last_line_start, _) = lines.last().unwrap();
            return Location {
                line,
                column: location - last_line_start,
            };
        });
        return Box::new(ParseError {
            error: format!("{}", error),
            location,
        });
    })?);
}
