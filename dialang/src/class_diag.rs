use std::collections::HashMap;

use crate::{Class, Attribute, Method};

pub(crate) fn make_class_diag(classes: HashMap<String, Class>) -> Vec<String> {
    let mut id = 2;
    let mut y = 25;

    class_diag_from_classes(&mut id, &mut y, classes)
}

fn class_diag_from_classes(id: &mut u32, y: &mut u32, classes: HashMap<String, Class>) -> Vec<String> {
    const Y_PADDING: u32 = 100;
    const X_PADDING: u32 = 15;
    let mut ret = Vec::new();
    for class in classes.values() {
        let Class { name, attributes, methods } = class;
        let attributes = get_attributes(attributes);
        let methods = get_methods(methods);
        let (a, y_tmp) = make_class(id, name, attributes, methods, X_PADDING, *y);
        *y = y_tmp + Y_PADDING;

        ret.push(a)
    };

    ret
}

fn get_attributes(attributes: &[Attribute]) -> Vec<String> {
    let mut v = Vec::new();
    for att in attributes {
        v.push(match &att.r#type {
            Some(r#type) => format!("{}: {}", att.name, r#type),
            None => att.name.clone(),
        })
    };

    v
}

fn get_methods(attributes: &[Method]) -> Vec<String> {
    let mut v = Vec::new();
    for att in attributes {
        v.push(match &att.ret_type {
            Some(ret_type) => format!("{}({}): {}", att.name, get_param_string(&att.parameters), ret_type),
            None => format!("{}({})", att.name, get_param_string(&att.parameters)),
        });
    };

    v
}

fn get_param_string(parameters: &[Attribute]) -> String {
    parameters.into_iter()
        .map(|att| match &att.r#type {
                Some(r#type) => format!("{}: {}", att.name, r#type),
                None => att.name.clone(),
        }).fold(String::new(), |l, r| {
            if l.len() > 0 {
                format!("{l}, {r}")
            } else {
                l
            }
        })
}

fn make_class(id: &mut u32, name: &str, attributes: Vec<String>, methods: Vec<String>, x_pos: u32, y_pos: u32) -> (String, u32) { // TODO: Calculate width
    const START_HEIGHT: u32 = 26;
    const ATTR_HEIGHT: u32 = 26;
    const SEPARATOR_HEIGHT: u32 = 8;
    const METHOD_HEIGHT: u32 = 26;
    const WIDTH: u32 = 230;
    let mut class = String::new();
    let mut m_id = *id+1;

    let mut y = START_HEIGHT;

    for attr in &attributes {
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

    for method in &methods {
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
