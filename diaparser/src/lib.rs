use token::Spanned;

mod lexer;
mod parser;
mod recovery_err;
mod token;
mod combinators;

pub struct ParserError;

#[derive(Debug)]
pub struct FuncCall {
    pub root: Spanned<String>,
    pub access: Option<Spanned<String>>,
    pub args: Vec<Spanned<String>>,
}

#[derive(Debug)]
pub struct Assignment {
    pub r#type: Option<Spanned<String>>,
    pub name: Spanned<String>,
    pub expr: Spanned<Expr>
}

#[derive(Debug)]
pub enum Expr {
    FuncCall(Box<FuncCall>),
    Assignment(Box<Assignment>),
    ExprList(Vec<Spanned<Self>>),
    Error
}

pub enum TopLevelStatement {
    Class(Class),
    AnnotatedBlock(AnnotatedBlock)
}

pub struct Class {
    pub name: Spanned<String>,
    pub attributes: Vec<Spanned<Attribute>>,
    pub methods: Vec<Spanned<Method>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Annotation {
    SequenceEntrypoint,
}

pub struct SequenceEntrypointBlock {
    pub function: Spanned<FuncCall>
}

pub enum AnnotatedBlock {
    SequenceEntrypoint(SequenceEntrypointBlock)
}

pub struct Method {
    pub name: Spanned<String>,
    pub parameters: Vec<Spanned<Attribute>>,
    pub ret_type: Option<Spanned<String>>,
    pub body: Option<Spanned<Expr>>
}

pub struct Attribute {
    pub name: Spanned<String>,
    pub r#type: Option<Spanned<String>>
}

pub fn tokenize(input: &str) -> Spanned<Result<Vec<Spanned<TopLevelStatement>>, ParserError>> {
    let errors = std::cell::RefCell::new(Vec::new());
    let toks = lexer::lex(&input, &errors);
    parser::token_parse(toks)
}
