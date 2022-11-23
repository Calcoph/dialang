use diaparser::Expr;

use crate::Spanned;

pub(crate) fn make_class_diag(tokens: Spanned<Expr>) -> String {
    let mut id = 2;
    let mut y = 25;

    class_diag_from_tokens(&mut id, &mut y, tokens)
}

fn class_diag_from_tokens(id: &mut u32, y: &mut u32, tokens: Spanned<Expr>) -> String {
    const Y_PADDING: u32 = 100;
    const X_PADDING: u32 = 15;
    match tokens.0 {
        Expr::Class { name, attributes, methods } => {
            let attributes = get_attributes(attributes);
            let methods = get_methods(methods);
            let (a, y_tmp) = make_class(id, &name.0, &attributes, &methods, X_PADDING, *y);
            *y = y_tmp + Y_PADDING;

            a
        },
        Expr::ExprList(e) => {
            let mut c = String::new();
            for exp in e {
                c += &class_diag_from_tokens(id, y, exp);
            }

            c
        },
        Expr::Error => todo!(),
        _ => unreachable!()
    }
}

fn get_attributes(attributes: Vec<Spanned<Expr>>) -> Vec<String> {
    let mut v = Vec::new();
    for att in attributes {
        v.push(match att.0 {
            Expr::Attribute { name, r#type } => {
                match r#type {
                    Some(r#type) => format!("{}: {}", name.0, r#type.0),
                    None => name.0,
                }
            },
            _ => unreachable!()
        })
    };

    v
}

fn get_methods(attributes: Vec<Spanned<Expr>>) -> Vec<String> {
    let mut v = Vec::new();
    for att in attributes {
        v.push(match att.0 {
            Expr::Method { name, parameters, ret_type } => {
                match ret_type {
                    Some(ret_type) => format!("{}({}): {}", name.0, get_param_string(parameters), ret_type.0),
                    None => format!("{}({})", name.0, get_param_string(parameters)),
                }
            },
            _ => unreachable!()
        })
    };

    v
}

fn get_param_string(parameters: Vec<Spanned<Expr>>) -> String {
    parameters.into_iter()
        .map(|exp| match exp.0 {
            Expr::Attribute { name, r#type } => match r#type {
                Some(r#type) => format!("{}: {}", name.0, r#type.0),
                None => name.0,
            },
            _ => unreachable!()
        }).fold(String::new(), |l, r| {
            if l.len() > 0 {
                format!("{l}, {r}")
            } else {
                l
            }
        })
}

fn make_class(id: &mut u32, name: &str, attributes: &[String], methods: &[String], x_pos: u32, y_pos: u32) -> (String, u32) { // TODO: Calculate width
    const START_HEIGHT: u32 = 26;
    const ATTR_HEIGHT: u32 = 26;
    const SEPARATOR_HEIGHT: u32 = 8;
    const METHOD_HEIGHT: u32 = 26;
    const WIDTH: u32 = 230;
    let mut class = String::new();
    let mut m_id = *id+1;

    let mut y = START_HEIGHT;

    for attr in attributes {
        class += &format!(
            include_str!("../../templates/cd-class/attribute.xml"),
            id=format!("class-diag-{m_id}"),
            value=attr,
            parent=format!("class-diag-{id}"),
            y=y
        );

        m_id += 1;
        y += ATTR_HEIGHT
    }
    class += &format!(
        include_str!("../../templates/cd-class/separator-bar.xml"),
        id=format!("class-diag-{m_id}"),
        parent=format!("class-diag-{id}"),
        y=y
    );
    m_id += 1;
    y += SEPARATOR_HEIGHT;

    for method in methods {
        class += &format!(
            include_str!("../../templates/cd-class/method.xml"),
            id=format!("class-diag-{m_id}"),
            value=method,
            parent=format!("class-diag-{id}"),
            y=y
        );

        m_id += 1;
        y += METHOD_HEIGHT
    }

    y += START_HEIGHT-METHOD_HEIGHT;

    let class = format!(
        include_str!("../../templates/cd-class/title.xml"),
        id=format!("class-diag-{id}"),
        value=name,
        parent="class-diag-1",
        x=x_pos,
        y=y_pos,
        width=WIDTH,
        height=y
    ) + &class;

    *id = m_id;

    (class, y)
}
