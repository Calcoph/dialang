use diaparser::Annotation;
use diaparser::Assignment;
use diaparser::FuncCall;
use diaparser::SequenceEntrypointBlock;
use diaparser::Expr as ParserExpr;
use diaparser::Class as ParserClass;
use diaparser::AnnotatedBlock as ParserAnnotatedBlock;
use diaparser::TopLevelStatement as ParserTopLevelStatement;
use diaparser::Method as ParserMethod;
use diaparser::Attribute as ParserAttribute;

pub(crate) enum Statement {
    Assignment { name: String, expr: Vec<Statement> },
    FuncCall { root: String, access: Option<String>, args: Vec<String> },
}

pub(crate) struct Method {
    pub(crate) name: String,
    pub(crate) parameters: Vec<Attribute>,
    pub(crate) ret_type: Option<String>,
    pub(crate) body: Vec<Statement>
}

pub(crate) struct Class {
    pub(crate) name: String,
    pub(crate) attributes: Vec<Attribute>,
    pub(crate) methods: Vec<Method>
}

pub(crate) struct AnnotatedBlock {
    pub(crate) annotation: Annotation,
    pub(crate) elements: Vec<ParserExpr>
}

pub(crate) enum TopLevelStatement {
    Class(Class),
    AnnotatedBlock(AnnotatedBlock)
}

pub(crate) struct Attribute {
    pub(crate) name: String,
    pub(crate) r#type: Option<String>
}

pub(crate) fn clean_parser_top_lvl_statement(stmnt: ParserTopLevelStatement) -> TopLevelStatement {
    match stmnt {
        ParserTopLevelStatement::Class(class) => TopLevelStatement::Class(clean_parser_class(class)),
        ParserTopLevelStatement::AnnotatedBlock(ablock) => TopLevelStatement::AnnotatedBlock(clean_parser_annotated_block(ablock)),
    }
}

pub(crate) fn clean_parser_annotated_block(ablock: ParserAnnotatedBlock) -> AnnotatedBlock {
    let (annotation, elements) = match ablock {
        ParserAnnotatedBlock::SequenceEntrypoint(SequenceEntrypointBlock {
            function: (function, _)
        }) => {
            (
                Annotation::SequenceEntrypoint,
                vec![ParserExpr::FuncCall(Box::new(function))]
            )
        },
    };

    AnnotatedBlock {
        annotation,
        elements
    }
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
        ParserExpr::FuncCall(f) => {
            let FuncCall{
                root: (root, _),
                access,
                args
            } = *f;

            vec![Statement::FuncCall {
                root,
                access: access.map(|(access, _)| access),
                args: args.into_iter().map(|(arg, _)| arg).collect()
            }]
        },
        ParserExpr::Assignment(a) =>{
            let Assignment {
                name: (name, _),
                expr,
                r#type
            } = *a;

            vec![Statement::Assignment {
                name,
                expr: parserexpr_to_statement(expr.0)
            }]
        },
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
