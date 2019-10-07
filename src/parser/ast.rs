#[derive(Debug)]
pub struct Ident(String);

impl Ident {
    pub fn new(id: &str) -> Self {
        return Self(id.into());
    }
}

#[derive(Debug)]
pub enum TypeReference {
    Resolve(Ident),
    Apply(Box<TypeReference>, Vec<TypeArgument>),
    Access(Box<TypeReference>, Ident),
    Bus(Option<Box<TypeReference>>, Box<Expression>),
}

#[derive(Debug)]
pub enum Expression {
    Resolve(Ident),
    DotAccess(Box<Expression>, Ident),
    BracketAccess(Box<Expression>, Box<Expression>),
    Vector(Vec<bool>),
    Integer(usize),
    BinaryExpression(Ident, Box<Expression>, Box<Expression>),
    Nested(Box<Expression>, Ident),
    Binary(Box<Expression>, Vec<BinaryTail>),
    Unary(Ident, Box<Expression>),
    TypeAccess(Box<TypeReference>, Ident),
}

#[derive(Debug)]
pub struct BinaryTail {
    pub operation: Ident,
    pub expression: Expression,
}

#[derive(Debug)]
pub enum IOPort {
    In(Ident, TypeReference),
    Out(Ident, TypeReference),
}

#[derive(Debug)]
pub enum Declaration {
    Circuit(Box<Circuit>),
    TypeAlias(Ident, Vec<TypeParameter>, TypeReference),
}

#[derive(Debug)]
pub enum TypeArgument {
    Value(Box<Expression>),
    Type(Box<TypeReference>),
}

#[derive(Debug)]
pub struct TypeParameter {
    name: Ident,
    constraints: Option<TypeReference>,
}

impl TypeParameter {
    pub fn new(name: Ident, constraints: Option<TypeReference>) -> Self {
        return Self { name, constraints };
    }
}

#[derive(Debug)]
pub struct Circuit {
    name: Ident,
    parameters: Vec<TypeParameter>,
    declarations: Vec<Declaration>,
}

impl Circuit {
    pub fn new(
        name: Ident,
        parameters: Option<Vec<TypeParameter>>,
        declarations: Vec<Declaration>,
    ) -> Self {
        return Self {
            name,
            parameters: parameters.unwrap_or(Vec::new()),
            declarations,
        };
    }
}

#[derive(Debug)]
pub struct Module {
    name: Ident,
    declarations: Vec<Declaration>,
}

impl Module {
    pub fn new(name: Ident, declarations: Vec<Declaration>) -> Self {
        return Self { name, declarations };
    }
}
