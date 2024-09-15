use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Node {
    Text(String),
    Element(ElementData),
}

impl Node {
    pub fn text(data: String) -> Self {
        Node::Text(data)
    }

    pub fn element(data: ElementData) -> Self {
        Node::Element(data)
    }
}

#[derive(Debug, PartialEq)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
    pub children: Vec<Node>,
}

pub type AttrMap = HashMap<String, String>;
