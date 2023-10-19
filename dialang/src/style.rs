use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Style {
    base_style: Option<BaseStyle>,
    stroke_color: Option<()>,
    stroke_width: Option<u32>,
    fill_color: Option<()>,
    alignment: Option<Alignment>,
    vertical_alignment: Option<VerticalAlignment>,
    spacing_top: Option<i32>,
    spacing_left: Option<i32>,
    spacing_right: Option<i32>,
    rotatable: Option<bool>,
    label_position: Option<RelativePosition>,
    points: Vec<[f32;2]>,
    port_constraint: Option<PortConstraint>,
    overflow: Option<Overflow>,
    font_style: Option<u32>,
    child_layout: Option<ChildLayout>,
    horizontal: Option<bool>,
    start_size: Option<u32>,
    horizontal_stack: Option<bool>,
    resize_parent: Option<bool>,
    resize_parent_max: Option<u32>,
    resize_last: Option<bool>,
    collapsible: Option<bool>,
    margin_bottom: Option<u32>
}

impl Style {
    pub fn new() -> StyleBuilder {
        StyleBuilder::new()
    }

    pub fn to_string(self) -> String {
        let mut string = "".to_string();
        if let Some(base_style) = self.base_style {
            string += &format!("{base_style};");
        }

        if let Some(font_style) = self.font_style {
            string += &format!("fontStyle={font_style};")
        }

        if let Some(base_style) = self.base_style {
            if base_style.stroke_color_able() {
                if let Some(stroke_color) = self.stroke_color {
                    todo!();
                    string += &format!("strokeColor=none;")
                } else {
                    string += &format!("strokeColor=none;")
                }
            }

            if base_style.stroke_width_able() {
                if let Some(stroke_width) = self.stroke_width {
                    string += &format!("strokeWidth={stroke_width};")
                }
            }

            if base_style.fill_color_able() {
                if let Some(fill_color) = self.fill_color {
                    todo!();
                    string += &format!("fillColor=none;")
                } else {
                    string += &format!("fillColor=none;")
                }
            }
        }

        if let Some(alignment) = self.alignment {
            string += &format!("align={alignment};")
        }

        if let Some(vertical_alignment) = self.vertical_alignment {
            string += &format!("verticalAlign={vertical_alignment};")
        }

        if let Some(spacing_top) = self.spacing_top {
            string += &format!("spacingTop={spacing_top};")
        }

        if let Some(spacing_left) = self.spacing_left {
            string += &format!("spacingLeft={spacing_left};")
        }

        if let Some(spacing_right) = self.spacing_right {
            string += &format!("spacingRight={spacing_right};")
        }

        if let Some(overflow) = self.overflow {
            string += &format!("overflow={overflow};")
        }

        if let Some(rotatable) = self.rotatable {
            let rotatable = bool_to_num(rotatable);
            string += &format!("rotatable={rotatable};")
        }

        if let Some(label_position) = self.label_position {
            string += &format!("labelPosition={label_position};")
        }

        if let Some(base_style) = self.base_style {
            if base_style.points_able() {
                let mut points = self.points.into_iter()
                    .fold("[".to_string(), |old, [x,y]| {
                        format!("{old}[{x},{y}],")
                    });
                if points.len() > 1 {
                    points.pop();
                }
                points += "]";
                string += &format!("points={points};")
            }
        }

        if let Some(port_constraint) = self.port_constraint {
            string += &format!("portConstraint={port_constraint};")
        }

        if let Some(child_layout) = self.child_layout {
            string += &format!("childLayout={child_layout};")
        }

        if let Some(horizontal) = self.horizontal {
            let horizontal = bool_to_num(horizontal);
            string += &format!("horizontal={horizontal};")
        }

        if let Some(start_size) = self.start_size {
            string += &format!("startSize={start_size};")
        }

        if let Some(horizontal_stack) = self.horizontal_stack {
            let horizontal_stack = bool_to_num(horizontal_stack);
            string += &format!("horizontalStack={horizontal_stack};")
        }

        if let Some(resize_parent) = self.resize_parent {
            let resize_parent = bool_to_num(resize_parent);
            string += &format!("resizeParent={resize_parent};")
        }

        if let Some(resize_parent_max) = self.resize_parent_max {
            string += &format!("resizeParentMax={resize_parent_max};")
        }

        if let Some(resize_last) = self.resize_last {
            let resize_last = bool_to_num(resize_last);
            string += &format!("resizeLast={resize_last};")
        }

        if let Some(collapsible) = self.collapsible {
            let collapsible = bool_to_num(collapsible);
            string += &format!("collapsible={collapsible};")
        }

        if let Some(margin_bottom) = self.margin_bottom {
            string += &format!("marginBottom={margin_bottom};")
        }

        string
    }

    pub fn default_line() -> Style {
        let style = StyleBuilder::new()
            .with_base_style(BaseStyle::Line)
            .with_stroke_width(1)
            .with_alignment(Alignment::Left)
            .with_vertical_alignment(VerticalAlignment::Middle)
            .with_spacing_top(-1)
            .with_spacing_left(3)
            .with_spacing_right(3)
            .with_rotatable(false)
            .with_label_position(RelativePosition::Right)
            .with_port_constraint(PortConstraint::EastWest)
            .build();

        debug_assert_eq!(style.clone().to_string(), "line;strokeWidth=1;fillColor=none;align=left;verticalAlign=middle;spacingTop=-1;spacingLeft=3;spacingRight=3;rotatable=0;labelPosition=right;points=[];portConstraint=eastwest;");

        style
    }

    pub fn default_text() -> Style {
        let style = StyleBuilder::new()
            .with_base_style(BaseStyle::Text)
            .with_alignment(Alignment::Left)
            .with_vertical_alignment(VerticalAlignment::Top)
            .with_spacing_left(4)
            .with_spacing_right(4)
            .with_overflow(Overflow::Hidden)
            .with_rotatable(false)
            .with_points(vec![[0.0,0.5],[1.0,0.5]])
            .with_port_constraint(PortConstraint::EastWest)
            .build();

        debug_assert_eq!(style.clone().to_string(), "text;strokeColor=none;fillColor=none;align=left;verticalAlign=top;spacingLeft=4;spacingRight=4;overflow=hidden;rotatable=0;points=[[0,0.5],[1,0.5]];portConstraint=eastwest;");

        style
    }

    pub fn default_swimlane() -> Style {
        //"swimlane;fontStyle=1;align=center;verticalAlign=top;childLayout=stackLayout;horizontal=1;startSize=26;horizontalStack=0;resizeParent=1;resizeParentMax=0;resizeLast=0;collapsible=1;marginBottom=0;"
        let style = StyleBuilder::new()
            .with_base_style(BaseStyle::SwimLane)
            .with_font_style(1)
            .with_alignment(Alignment::Center)
            .with_vertical_alignment(VerticalAlignment::Top)
            .with_child_layout(ChildLayout::Stack)
            .with_horizontal(true)
            .with_start_size(26)
            .with_horizontal_stack(false)
            .with_resize_parent(true)
            .with_resize_parent_max(0)
            .with_resize_last(false)
            .with_collapsible(true)
            .with_margin_bottom(0)
            .build();

        debug_assert_eq!(style.clone().to_string(), "swimlane;fontStyle=1;align=center;verticalAlign=top;childLayout=stackLayout;horizontal=1;startSize=26;horizontalStack=0;resizeParent=1;resizeParentMax=0;resizeLast=0;collapsible=1;marginBottom=0;");

        style
    }

    pub fn default_actor() -> Style {
        let style = StyleBuilder::new()
            .with_shape(Shape::UMLActor)
            .with_vertical_label_position(RelativePosition::Bottom)
            .with_vertical_alignment(VerticalAlignment::Top)
            .with_html(true)
            .with_outline_connect(false)
            .build();

        debug_assert_eq!(style.clone().to_string(), "shape=umlActor;verticalLabelPosition=bottom;verticalAlign=top;html=1;outlineConnect=0;");

        style
    }

    pub fn default_seq_class() -> Style {
        let style = StyleBuilder::new()
            .with_rounded(false)
            .with_white_space(WhiteSpace::Wrap)
            .with_html(true)
            .build();

        debug_assert_eq!(style.clone().to_string(), "rounded=0;whiteSpace=wrap;html=1;");

        style
    }

    pub(crate) fn default_lifetime_line() -> Style {
        let style = StyleBuilder::new()
            .with_html(true)
            .with_entryx(0.5)
            .with_entryy(1.0)
            .with_entrydx(0.0)
            .with_entrydy(0.0)
            .build();

        debug_assert_eq!(style.clone().to_string(), "endArrow=none;html=1;entryX=0.5;entryY=1;entryDx=0;entryDy=0;");

        style
    }

    pub(crate) fn default_call_arrow() -> Style {
        let style = StyleBuilder::new()
            .build();

        debug_assert_eq!(style.clone().to_string(), "endArrow=classic;html=1;entryX=0.25;entryY=0;entryDx=0;entryDy=0;");

        style
    }

    pub(crate) fn default_call_text() -> Style {
        let style = StyleBuilder::new()
            .build();

        debug_assert_eq!(style.clone().to_string(), "edgeLabel;html=1;align=center;verticalAlign=middle;resizable=0;points=[];");

        style
    }

    pub(crate) fn default_return_arrow() -> Style {
        let style = StyleBuilder::new()
            .build();

        debug_assert_eq!(style.clone().to_string(), "endArrow=classic;html=1;exitX=0;exitY=1;exitDx=0;exitDy=0;dashed=1;");

        style
    }

    pub(crate) fn default_return_text() -> Style {
        Self::default_call_text()
    }
}

fn bool_to_num(b: bool) -> u32 {
    if b {
        1
    } else {
        0
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WhiteSpace {
    Wrap
}

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    UMLActor
}

#[derive(Debug, Clone, Copy)]
pub enum BaseStyle {
    Line,
    Text,
    SwimLane
}

impl BaseStyle {
    fn stroke_color_able(self) -> bool {
        match self {
            BaseStyle::Line => false,
            BaseStyle::Text => true,
            BaseStyle::SwimLane => false,
        }
    }

    fn fill_color_able(self) -> bool {
        match self {
            BaseStyle::Line => true,
            BaseStyle::Text => true,
            BaseStyle::SwimLane => false,
        }
    }

    fn stroke_width_able(self) -> bool {
        match self {
            BaseStyle::Line => true,
            BaseStyle::Text => false,
            BaseStyle::SwimLane => false,
        }
    }

    fn points_able(self) -> bool {
        match self {
            BaseStyle::Line => true,
            BaseStyle::Text => true,
            BaseStyle::SwimLane => false,
        }
    }
}

impl Display for BaseStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseStyle::Line => write!(f, "line"),
            BaseStyle::Text => write!(f, "text"),
            BaseStyle::SwimLane => write!(f, "swimlane"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Left,
    Center
}

impl Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Alignment::Left => write!(f, "left"),
            Alignment::Center => write!(f, "center"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VerticalAlignment {
    Top,
    Middle
}

impl Display for VerticalAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerticalAlignment::Top => write!(f, "top"),
            VerticalAlignment::Middle => write!(f, "middle"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RelativePosition {
    Right,
    Bottom
}

impl Display for RelativePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelativePosition::Right => write!(f, "right"),
            RelativePosition::Bottom => write!(f, "bottom"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PortConstraint {
    EastWest
}

impl Display for PortConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PortConstraint::EastWest => write!(f, "eastwest"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Overflow {
    Hidden
}

impl Display for Overflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Overflow::Hidden => write!(f, "hidden"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ChildLayout {
    Stack
}

impl Display for ChildLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChildLayout::Stack => write!(f, "stackLayout"),
        }
    }
}

pub struct StyleBuilder {
    base_style: Option<BaseStyle>,
    stroke_color: Option<()>,
    stroke_width: Option<u32>,
    fill_color: Option<()>,
    alignment: Option<Alignment>,
    vertical_alignment: Option<VerticalAlignment>,
    spacing_top: Option<i32>,
    spacing_left: Option<i32>,
    spacing_right: Option<i32>,
    rotatable: Option<bool>,
    label_position: Option<RelativePosition>,
    points: Vec<[f32;2]>,
    port_constraint: Option<PortConstraint>,
    overflow: Option<Overflow>,
    font_style: Option<u32>,
    child_layout: Option<ChildLayout>,
    horizontal: Option<bool>,
    start_size: Option<u32>,
    horizontal_stack: Option<bool>,
    resize_parent: Option<bool>,
    resize_parent_max: Option<u32>,
    resize_last: Option<bool>,
    collapsible: Option<bool>,
    margin_bottom: Option<u32>,
    shape: Option<Shape>,
    vertical_label_position: Option<RelativePosition>,
    html: Option<bool>,
    outline_connect: Option<bool>,
    rounded: Option<bool>,
    white_space: Option<WhiteSpace>,
    entryx: Option<f64>,
    entryy: Option<f64>,
    entrydx: Option<f64>,
    entrydy: Option<f64>,
}

impl StyleBuilder {
    pub fn new() -> StyleBuilder {
        StyleBuilder {
            base_style: None,
            stroke_color: None,
            stroke_width: None,
            fill_color: None,
            alignment: None,
            vertical_alignment: None,
            spacing_top: None,
            spacing_left: None,
            spacing_right: None,
            rotatable: None,
            label_position: None,
            points: Vec::new(),
            port_constraint: None,
            overflow: None,
            font_style: None,
            child_layout: None,
            horizontal: None,
            start_size: None,
            horizontal_stack: None,
            resize_parent: None,
            resize_parent_max: None,
            resize_last: None,
            collapsible: None,
            margin_bottom: None,
            shape: None,
            vertical_label_position: None,
            html: None,
            outline_connect: None,
            rounded: None,
            white_space: None,
            entryx: None,
            entryy: None,
            entrydx: None,
            entrydy: None,
        }
    }

    pub fn with_base_style(mut self, base_style: BaseStyle) -> Self {
        self.base_style = Some(base_style);

        self
    }

    pub fn with_stroke_color(mut self, stroke_color: ()) -> Self {
        self.stroke_color = Some(stroke_color);

        self
    }

    pub fn with_stroke_width(mut self, stroke_width: u32) -> Self {
        self.stroke_width = Some(stroke_width);

        self
    }

    pub fn with_fill_color(mut self, fill_color: ()) -> Self {
        self.fill_color = Some(fill_color);

        self
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = Some(alignment);

        self
    }

    pub fn with_vertical_alignment(mut self, vertical_alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = Some(vertical_alignment);

        self
    }

    pub fn with_spacing_top(mut self, spacing_top: i32) -> Self {
        self.spacing_top = Some(spacing_top);

        self
    }

    pub fn with_spacing_left(mut self, spacing_left: i32) -> Self {
        self.spacing_left = Some(spacing_left);

        self
    }

    pub fn with_spacing_right(mut self, spacing_right: i32) -> Self {
        self.spacing_right = Some(spacing_right);

        self
    }

    pub fn with_rotatable(mut self, rotatable: bool) -> Self {
        self.rotatable = Some(rotatable);

        self
    }

    pub fn with_label_position(mut self, label_position: RelativePosition) -> Self {
        self.label_position = Some(label_position);

        self
    }

    pub fn with_points(mut self, points: Vec<[f32;2]>) -> Self {
        self.points = points;

        self
    }

    pub fn with_port_constraint(mut self, port_constraint: PortConstraint) -> Self {
        self.port_constraint = Some(port_constraint);

        self
    }

    pub fn with_overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);

        self
    }

    pub fn with_font_style(mut self, font_style: u32) -> Self {
        self.font_style = Some(font_style);

        self
    }

    pub fn with_child_layout(mut self, child_layout: ChildLayout) -> Self {
        self.child_layout = Some(child_layout);

        self
    }

    pub fn with_horizontal(mut self, horizontal: bool) -> Self {
        self.horizontal = Some(horizontal);

        self
    }

    pub fn with_start_size(mut self, start_size: u32) -> Self {
        self.start_size = Some(start_size);

        self
    }

    pub fn with_horizontal_stack(mut self, horizontal_stack: bool) -> Self {
        self.horizontal_stack = Some(horizontal_stack);

        self
    }

    pub fn with_resize_parent(mut self, resize_parent: bool) -> Self {
        self.resize_parent = Some(resize_parent);

        self
    }

    pub fn with_resize_parent_max(mut self, resize_parent_max: u32) -> Self {
        self.resize_parent_max = Some(resize_parent_max);

        self
    }

    pub fn with_resize_last(mut self, resize_last: bool) -> Self {
        self.resize_last = Some(resize_last);

        self
    }

    pub fn with_collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = Some(collapsible);

        self
    }

    pub fn with_margin_bottom(mut self, margin_bottom: u32) -> Self {
        self.margin_bottom = Some(margin_bottom);

        self
    }

    pub fn build(self) -> Style {
        Style {
            base_style: self.base_style,
            stroke_color: self.stroke_color,
            stroke_width: self.stroke_width,
            fill_color: self.fill_color,
            alignment: self.alignment,
            vertical_alignment: self.vertical_alignment,
            spacing_top: self.spacing_top,
            spacing_left: self.spacing_left,
            spacing_right: self.spacing_right,
            rotatable: self.rotatable,
            label_position: self.label_position,
            points: self.points,
            port_constraint: self.port_constraint,
            overflow: self.overflow,
            font_style: self.font_style,
            child_layout: self.child_layout,
            horizontal: self.horizontal,
            start_size: self.start_size,
            horizontal_stack: self.horizontal_stack,
            resize_parent: self.resize_parent,
            resize_parent_max: self.resize_parent_max,
            resize_last: self.resize_last,
            collapsible: self.collapsible,
            margin_bottom: self.margin_bottom,
        }
    }

    fn with_shape(mut self, shape: Shape) -> Self {
        self.shape = Some(shape);

        self
    }

    fn with_vertical_label_position(mut self, vertical_label_position: RelativePosition) -> Self {
        self.vertical_label_position = Some(vertical_label_position);

        self
    }

    fn with_html(mut self, html: bool) -> Self {
        self.html = Some(html);

        self
    }

    fn with_outline_connect(mut self, outline_connect: bool) -> Self {
        self.outline_connect = Some(outline_connect);

        self
    }

    fn with_rounded(mut self, rounded: bool) -> Self {
        self.rounded = Some(rounded);

        self
    }

    fn with_white_space(mut self, white_space: WhiteSpace) -> Self {
        self.white_space = Some(white_space);

        self
    }

    fn with_entryx(mut self, entryx: f64) -> Self {
        self.entryx = Some(entryx);

        self
    }

    fn with_entryy(mut self, entryy: f64) -> Self {
        self.entryy = Some(entryy);

        self
    }

    fn with_entrydx(mut self, entrydx: f64) -> Self {
        self.entrydx = Some(entrydx);

        self
    }

    fn with_entrydy(mut self, entrydy: f64) -> Self {
        self.entrydy = Some(entrydy);

        self
    }
}
