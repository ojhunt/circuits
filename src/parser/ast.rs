#[derive(Debug)]
pub struct Ident(String);

impl Ident {
    pub fn new(id: &str) -> Self {
        return Self(id.into());
    }
}

#[derive(Debug)]
pub enum TypeRef {
    Resolve(Ident),
    Apply(Box<TypeRef>, Vec<Expression>),
    Nested(Box<TypeRef>, Ident),
}

#[derive(Debug)]
pub enum Expression {
    Resolve(Ident),
    DotAccess(Box<Expression>, Ident),
    BracketAccess(Box<Expression>, Box<Expression>),
    Vector(Vec<bool>),
    Integer(usize),
    BinaryExpression(Ident, Box<Expression>, Box<Expression>),
}

#[derive(Debug)]
pub enum Declaration {
    Circuit(Box<Circuit>),
    TypeAlias(Ident, Vec<TypeParameter>, TypeRef),
}

#[derive(Debug)]
pub struct TypeArgument {
    expr: Box<Expression>,
}

#[derive(Debug)]
pub struct TypeParameter {
    name: Ident,
    constraints: Option<TypeRef>,
}

impl TypeParameter {
    pub fn new(name: Ident, constraints: Option<TypeRef>) -> Self {
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
