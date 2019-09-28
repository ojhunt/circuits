#[derive(Debug)]
pub struct Ident(String);

impl Ident {
    pub fn new(id: &str) -> Self {
        return Self(id.into());
    }
}

pub enum TypeRef {}

#[derive(Debug)]
pub enum Expression {
    Resolve(Ident),
    DotAccess(Box<Expression>, Ident),
    BracketAccess(Box<Expression>, Box<Expression>),
    Literal(Vec<bool>),
    BinaryExpression(Ident, Box<Expression>, Box<Expression>),
}

pub struct CircuitArgument(Box<Expression>);
pub struct CircuitParameter(Ident, Option<TypeRef>);

struct Circuit {
    name: String,
    parameters: Vec<CircuitParameter>,
}
