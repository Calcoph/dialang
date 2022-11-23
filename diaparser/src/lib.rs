use token::Spanned;

mod lexer;
mod parser;
mod recovery_err;
mod token;
mod combinators;

#[derive(Debug)]
pub enum Expr {
    Class {
        name: Spanned<String>,
        attributes: Vec<Spanned<Self>>,
        methods: Vec<Spanned<Self>>,
    },
    Attribute {
        name: Spanned<String>,
        r#type: Option<Spanned<String>>
    },
    Method {
        name: Spanned<String>,
        parameters: Vec<Spanned<Self>>,
        ret_type: Option<Spanned<String>>
    },
    ExprList(Vec<Spanned<Self>>),
    Error
}

pub fn tokenize(input: &str) -> Spanned<Expr> {
    let errors = std::cell::RefCell::new(Vec::new());
    let toks = lexer::lex(&input, &errors);
    parser::token_parse(toks)
}
