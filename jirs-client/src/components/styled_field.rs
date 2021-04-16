use seed::prelude::*;
use seed::*;

use crate::shared::ToNode;
use crate::Msg;

#[derive(Debug)]
pub struct StyledField<'l> {
    pub label: &'l str,
    pub tip: Option<&'l str>,
    pub input: Node<Msg>,
    pub class_list: &'l str,
}

impl<'l> Default for StyledField<'l> {
    fn default() -> Self {
        Self {
            label: "",
            tip: None,
            input: Node::Empty,
            class_list: "",
        }
    }
}

impl<'l> ToNode for StyledField<'l> {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

pub fn render(values: StyledField) -> Node<Msg> {
    let StyledField {
        label,
        tip,
        input,
        class_list,
    } = values;
    let tip_node = tip.map(|s| div![C!["styledTip"], s]).unwrap_or(empty![]);

    div![
        C!["styledField", class_list],
        seed::label![C!["styledLabel"], label],
        input,
        tip_node,
    ]
}
