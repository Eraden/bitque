use {
    crate::{shared::ToNode, Msg},
    seed::{prelude::*, *},
};

#[derive(Debug)]
pub struct StyledField<'l> {
    label: &'l str,
    tip: Option<&'l str>,
    input: Node<Msg>,
    class_list: Vec<&'l str>,
}

impl<'l> StyledField<'l> {
    pub fn build() -> StyledFieldBuilder<'l> {
        StyledFieldBuilder::default()
    }
}

impl<'l> ToNode for StyledField<'l> {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

#[derive(Default, Debug)]
pub struct StyledFieldBuilder<'l> {
    label: Option<&'l str>,
    tip: Option<&'l str>,
    input: Option<Node<Msg>>,
    class_list: Vec<&'l str>,
}

impl<'l> StyledFieldBuilder<'l> {
    pub fn label(mut self, label: &'l str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn tip(mut self, tip: &'l str) -> Self {
        self.tip = Some(tip);
        self
    }

    pub fn input(mut self, input: Node<Msg>) -> Self {
        self.input = Some(input);
        self
    }

    pub fn add_class(mut self, name: &'l str) -> Self {
        self.class_list.push(name);
        self
    }

    pub fn build(self) -> StyledField<'l> {
        StyledField {
            label: self.label.unwrap_or_default(),
            tip: self.tip,
            input: self.input.unwrap_or(empty![]),
            class_list: self.class_list,
        }
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
        attrs![At::Class => class_list.join(" "), At::Class => "styledField"],
        seed::label![attrs![At::Class => "styledLabel"], label],
        input,
        tip_node,
    ]
}
