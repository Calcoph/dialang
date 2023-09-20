use std::{process::exit, collections::HashMap};

use options::Options;

use diaparser::Expr as ParserExpr;
use diaparser::Class as ParserClass;
use diaparser::Method as ParserMethod;
use diaparser::Attribute as ParserAttribute;

use crate::class_diag::make_class_diag;

mod class_diag;
mod options;

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

    let classes = make_class_diag(classes);
    let class_diag = match opt.class_diag {
        true => {
            classes.iter()
                .map(|c| c.to_string())
                .collect::<String>()
        },
        false => "".to_string(),
    };

    let file = format!(include_str!("../../templates/doc.xml"), seq_diag="", comm_diag="", class_diag=class_diag);
    std::fs::write(opt.output_path, file).unwrap();
}

struct Method {
    name: String,
    parameters: Vec<Attribute>,
    ret_type: Option<String>,
    body: Option<ParserExpr>
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
        Some((body, _)) => Some(body),
        None => None,
    };
    
    Method {
        name,
        parameters,
        ret_type,
        body
    }
}
