use std::collections::HashMap;

use inline_xml::{xml_tag, Tag};

use crate::{Class, Attribute, Method};

const START_HEIGHT: u32 = 26;
const ATTR_HEIGHT: u32 = 26;
const SEPARATOR_HEIGHT: u32 = 8;
const METHOD_HEIGHT: u32 = 26;
const CLASS_WIDTH: u32 = 230;

pub(crate) fn make_class_diag(classes: HashMap<String, Class>) -> Vec<String> {
    let mut id = 2;
    let mut y = 25;

    class_diag_from_classes(&mut id, &mut y, classes)
}

fn class_diag_from_classes(id: &mut u32, y: &mut u32, classes: HashMap<String, Class>) -> Vec<String> {
    const Y_PADDING: u32 = 30;
    const X_PADDING: u32 = 15;
    let mut ret = Vec::new();
    for class in classes.values() {
        let Class { name, attributes, methods } = class;
        let attributes = get_attributes(attributes);
        let methods = get_methods(methods);
        let (a, y_tmp) = make_class(id, name, attributes, methods, X_PADDING, *y);
        *y += y_tmp + Y_PADDING;

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
    let mut class_v = Vec::new();
    let mut m_id = *id+1;

    let mut y = START_HEIGHT;

    for attr in attributes {
        let attrib = make_attribute(
            format!("class-diag-{m_id}"),
            attr,
            format!("class-diag-{id}"),
            y
        );
        class_v.push(attrib);

        m_id += 1;
        y += ATTR_HEIGHT
    }
    class_v.push(make_separator_bar(
        format!("class-diag-{m_id}"),
        format!("class-diag-{id}"),
        y
    ));
    m_id += 1;
    y += SEPARATOR_HEIGHT;

    for method in methods {
        class_v.push(make_method(
            format!("class-diag-{m_id}"),
            method,
            format!("class-diag-{id}"),
            y
        ));

        m_id += 1;
        y += METHOD_HEIGHT
    }
    let class_title = make_class_title(
        format!("class-diag-{id}"),
        name,
        "class-diag-1",
        x_pos,
        y_pos,
        y
    );

    let class = class_v.into_iter()
        .map(|elem| elem.to_string())
        .fold(class_title.to_string(), |old_str, new_elem| old_str + &new_elem);

    *id = m_id;

    (class, y)
}

fn make_separator_bar(id: String, parent: String, y: u32) -> Tag {
    let value = "";
    let style = "line;strokeWidth=1;fillColor=none;align=left;verticalAlign=middle;spacingTop=-1;spacingLeft=3;spacingRight=3;rotatable=0;labelPosition=right;points=[];portConstraint=eastwest;";
    let vertex = "1";
    let width = CLASS_WIDTH;
    let height = SEPARATOR_HEIGHT;
    let r#as = "geometry";

    xml_tag!(
        <mxCell id={id} value={value} style={style} vertex={vertex} parent={parent}>
            <mxGeometry y={y} width={width} height={height} as={r#as}/>
        </mxCell>
    )
}

fn make_attribute(id: String, value: String, parent: String, y: u32) -> Tag {
    let style = "text;strokeColor=none;fillColor=none;align=left;verticalAlign=top;spacingLeft=4;spacingRight=4;overflow=hidden;rotatable=0;points=[[0,0.5],[1,0.5]];portConstraint=eastwest;";
    let vertex = "1";
    let width = CLASS_WIDTH;
    let height = ATTR_HEIGHT;
    let r#as = "geometry";

    xml_tag!(
        <mxCell id={id} value={value} style={style} vertex={vertex} parent={parent}>
            <mxGeometry y={y} width={width} height={height} as={r#as}/>
        </mxCell>
    )
}

fn make_method(id: String, value: String, parent: String, y: u32) -> Tag {
    let style = "text;strokeColor=none;fillColor=none;align=left;verticalAlign=top;spacingLeft=4;spacingRight=4;overflow=hidden;rotatable=0;points=[[0,0.5],[1,0.5]];portConstraint=eastwest;";
    let vertex = "1";
    let width = CLASS_WIDTH;
    let height = METHOD_HEIGHT;
    let r#as = "geometry";

    xml_tag!(
        <mxCell id={id} value={value} style={style} vertex={vertex} parent={parent}>
            <mxGeometry y={y} width={width} height={height} as={r#as}/>
        </mxCell>
    )
}

fn make_class_title(id: String, value: &str, parent: &str, x: u32, y: u32, height: u32) -> Tag {
    let style = "swimlane;fontStyle=1;align=center;verticalAlign=top;childLayout=stackLayout;horizontal=1;startSize=26;horizontalStack=0;resizeParent=1;resizeParentMax=0;resizeLast=0;collapsible=1;marginBottom=0;";
    let vertex = "1";
    let width = CLASS_WIDTH;
    let r#as = "geometry";

    xml_tag!(
        <mxCell id={id} value={value} style={style} vertex={vertex} parent={parent}>
            <mxGeometry x={x} y={y} width={width} height={height} as={r#as}/>
        </mxCell>
    )
}
