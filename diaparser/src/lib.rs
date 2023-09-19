use token::Spanned;

mod lexer;
mod parser;
mod recovery_err;
mod token;
mod combinators;

pub struct ParserError;

#[derive(Debug)]
pub enum Expr {
    FuncCall {
        root: Spanned<String>,
        access: Option<Spanned<String>>,
        args: Vec<Spanned<String>>,
    },
    Assignment {
        name: Spanned<String>,
        expr: Box<Spanned<Self>>
    },
    ExprList(Vec<Spanned<Self>>),
    Error
}

pub struct Class {
    pub name: Spanned<String>,
    pub attributes: Vec<Spanned<Attribute>>,
    pub methods: Vec<Spanned<Method>>,
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

pub fn tokenize(input: &str) -> Spanned<Result<Vec<Spanned<Class>>, ParserError>> {
    let errors = std::cell::RefCell::new(Vec::new());
    let toks = lexer::lex(&input, &errors);
    parser::token_parse(toks)
}
