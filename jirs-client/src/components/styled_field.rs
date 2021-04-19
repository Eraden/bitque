use seed::prelude::*;
use seed::*;

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

impl<'l> StyledField<'l> {
    pub fn render(self) -> Node<Msg> {
        div![
            C!["styledField", self.class_list],
            seed::label![C!["styledLabel"], self.label],
            self.input,
            self.tip
                .map_or_else(|| empty![], |s| div![C!["styledTip"], s]),
        ]
    }
}
