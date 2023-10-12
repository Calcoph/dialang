use std::{process::exit, collections::HashMap};

use options::Options;

use diaparser::Expr as ParserExpr;
use diaparser::Class as ParserClass;
use diaparser::Method as ParserMethod;
use diaparser::Attribute as ParserAttribute;

use crate::class_diag::make_class_diag;

mod seq_diag;
mod class_diag;
mod options;
mod style;

fn main() {
    let opt = Options::load();
    if !opt.eval {
        exit(0)
    }

    let input = std::fs::read_to_string(opt.input_path).unwrap();
    let (tokens, _) = diaparser::tokenize(&input);

    let tokens = match tokens {
        Ok(t) => t,
        Err(_) => panic!("Error while parsing the file, check that the syntax is correct.\nThis message is temporary, there will be better ones in the future."),
    };

    let classes = tokens.into_iter()
        .map(|(class, _)| clean_parser_class(class))
        .map(|class| (class.name.clone(), class))
        .collect::<HashMap<_,_>>();

    let class_diag = if opt.class_diag {
        let classes = make_class_diag(classes);
        classes.into_iter()
            .collect::<String>()
    } else {
        "".to_string()
    };

    let seq_diag = if opt.seq_diag {
        "".to_string()
    } else {
        "".to_string()
    };

    let comm_diag = if opt.comm_diag {
        "".to_string()
    } else {
        "".to_string()
    };

    let file = format!(include_str!("../../templates/doc.xml"), seq_diag=seq_diag, comm_diag=comm_diag, class_diag=class_diag);
    std::fs::write(opt.output_path, file).unwrap();
}

enum Statement {
    Assignment { name: String, expr: Vec<Statement> },
    FuncCall { root: String, access: Option<String>, args: Vec<String> },
}

struct Method {
    name: String,
    parameters: Vec<Attribute>,
    ret_type: Option<String>,
    body: Vec<Statement>
}

struct Class {
    name: String,
    attributes: Vec<Attribute>,
    methods: Vec<Method>
}

struct Attribute {
    name: String,
    r#type: Option<String>
}

fn clean_parser_class(class: ParserClass) -> Class {
    let ParserClass {
        name: (name, _),
        attributes,
        methods
    } = class;
    let attributes = attributes.into_iter().map(|(attr, _)| {
        clean_parser_attribute(attr)
    }).collect();

    let methods = methods.into_iter().map(|(method, _)| {
        clean_parser_method(method)
    }).collect();

    Class {
        name,
        attributes,
        methods
    }
}

fn clean_parser_attribute(attribute: ParserAttribute) -> Attribute {
    let ParserAttribute {
        name: (name, _),
        r#type,
    } = attribute;

    let r#type = match r#type {
        Some((ty, _)) => Some(ty),
        None => None,
    };

    Attribute {
        name,
        r#type
    }
}

fn clean_parser_method(method: ParserMethod) -> Method {
    let ParserMethod {
        name: (name, _),
        parameters,
        ret_type,
        body,
    } = method;

    let parameters = parameters.into_iter()
        .map(|(attr, _)| clean_parser_attribute(attr)).collect();

    let ret_type = match ret_type {
        Some((ty, _)) => Some(ty),
        None => None,
    };

    let body = match body {
        Some((body, _)) => parserexpr_to_statement(body),
        None => vec![],
    };

    Method {
        name,
        parameters,
        ret_type,
        body
    }
}

fn parserexpr_to_statement(expr: ParserExpr) -> Vec<Statement> {
    match expr {
        ParserExpr::FuncCall { root: (root, _), access, args } => vec![
            Statement::FuncCall {
                root,
                access: access.map(|(access, _)| access),
                args: args.into_iter().map(|(arg, _)| arg).collect()
            }
        ],
        ParserExpr::Assignment { name: (name, _), expr } => vec![
            Statement::Assignment {
                name,
                expr: parserexpr_to_statement(expr.0)
            }
        ],
        ParserExpr::ExprList(expr_list) => {
            let mut v = Vec::new();
            for (expr, _) in expr_list {
                v.extend(parserexpr_to_statement(expr).into_iter())
            }
            v
        },
        ParserExpr::Error => vec![],
    }
}
