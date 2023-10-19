use inline_xml::{xml_tag, Tag};

use crate::style::Style;

fn make_actor(id: &str, parent: &str) -> Tag {
    let value = "Actor";
    let style = Style::default_actor().to_string();
    let vertex = "1";
    let x = "55";
    let y = "220";
    let width = "30";
    let height = "60";
    let r#as = "geometry";

    xml_tag!(
        <mxCell id={id} value={value} style={style} vertex={vertex} parent={parent}>
            <mxGeometry x={x} y={y} width={width} height={height} as={r#as}/>
        </mxCell>
    )
}

fn make_instance(id: &str, parent: &str) -> Tag {
    /*
    <mxCell id="AYnkgXlg9PiY_2FhFIBn-1" value="Class1" style="rounded=0;whiteSpace=wrap;html=1;" parent="seq-diag-1" vertex="1">
        <mxGeometry x="190" y="220" width="120" height="60" as="geometry"/>
    </mxCell>
    */

    let value = "ClassName";
    let style = Style::default_seq_class().to_string();
    let vertex = "1";
    let x = "190";
    let y = "220";
    let width = "120";
    let height = "60";
    let r#as = "geometry";

    xml_tag!(
        <mxCell id={id} value={value} style={style} parent={parent} vertex={vertex}>
            <mxGeometry x={x} y={y} width={width} height={height} as={r#as}/>
        </mxCell>
    )
}

fn make_lifetime_line(id: &str, parent: &str, target: &str) -> Tag {
    let value = "";
    let style = Style::default_lifetime_line().to_string();
    let edge = "1";
    let width = "50";
    let height = "50";
    let relative = "1";
    let as1 = "geometry";
    let x1 = "250";
    let y1 = "640";
    let as2 = "sourcePoint";
    let x2 = "450";
    let y2 = "380";
    let as3 = "targetPoint";

    xml_tag!(
        <mxCell id={id} value={value} style={style} parent={parent} target={target} edge={edge}>
            <mxGeometry width={width} height={height} relative={relative} as={as1}>
                <mxPoint x={x1} y={y1} as={as2}/>
                <mxPoint x={x2} y={y2} as={as3}/>
            </mxGeometry>
        </mxCell>
    )
}

fn make_call_line(id: &str, text_id: &str, parent: &str, text: &str, target: &str) -> (Tag, Tag) {
    let value = "";
    let style = Style::default_call_arrow().to_string();
    let text_style = Style::default_call_text().to_string();
    let vertex = "1";
    let edge = "1";
    let connectable = "0";
    let relative = "1";
    let text_relative = "1";
    let arrow_as1 = "geometry";
    let text_as1 = "geometry";
    let text_as2 = "offset";
    let text_x = "0.1029";
    let width = "50";
    let height = "50";
    let x1 = "70";
    let x2 = "450";
    let y1 = "320";
    let y2 = "380";
    let arrow_as2 = "sourcePoint";
    let arrow_as3 = "targetPoint";

    (
        xml_tag!(
            <mxCell id={id} value={value} style={style} parent={parent} target={target} edge={edge}>
                <mxGeometry width={width} height={height} relative={relative} as={arrow_as1}>
                    <mxPoint x={x1} y={y1} as={arrow_as2}/>
                    <mxPoint x={x2} y={y2} as={arrow_as3}/>
                </mxGeometry>
            </mxCell>
        ),
        xml_tag!(
            <mxCell id={text_id} value={text} style={text_style} parent={id} vertex={vertex} connectable={connectable}>
                <mxGeometry x={text_x} relative={text_relative} as={text_as1}>
                    <mxPoint as={text_as2}/>
                </mxGeometry>
            </mxCell>
        )
    )
}

fn make_return_line(id: &str, text_id: &str, parent: &str, text: &str, target: &str, source: &str) -> (Tag, Tag) {
    let style = Style::default_return_arrow().to_string();
    let text_style = Style::default_return_text().to_string();
    let width = "50";
    let height = "50";
    let relative = "1";
    let text_relative = "1";
    let as1 = "geometry";
    let as2 = "sourcePoint";
    let as3 = "targetPoint";
    let text_as1 = "geometry";
    let text_as2 = "offset";
    let x1 = "300";
    let x2 = "70";
    let y1 = "450";
    let y2 = "510";
    let text_x = "-0.1157";
    let edge = "1";
    let vertex = "1";
    let connectable = "0";
    let value = "";

    (
        xml_tag!(
            <mxCell id={id} value={value} style={style} parent={parent} source={source} edge={edge}>
                <mxGeometry width={width} height={height} relative={relative} as={as1}>
                    <mxPoint x={x1} y={y1} as={as2}/>
                    <mxPoint x={x2} y={y2} as={as3}/>
                </mxGeometry>
            </mxCell>
        ),
        xml_tag!(
            <mxCell id={text_id} value={text} style={text_style} parent={id} vertex={vertex} connectable={connectable}>
                <mxGeometry x={text_x} relative={text_relative} as={text_as1}>
                    <mxPoint as={text_as2}/>
                </mxGeometry>
            </mxCell>
        )
    )
}
