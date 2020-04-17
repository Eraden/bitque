use seed::{prelude::*, *};

use crate::shared::ToNode;
use crate::Msg;

pub struct StyledLink {
    children: Vec<Node<Msg>>,
    class_list: Vec<String>,
    href: String,
}

impl StyledLink {
    pub fn build() -> StyledLinkBuilder {
        StyledLinkBuilder::default()
    }
}

#[derive(Default)]
pub struct StyledLinkBuilder {
    children: Vec<Node<Msg>>,
    class_list: Vec<String>,
    href: String,
}

impl StyledLinkBuilder {
    pub fn add_child(mut self, child: Node<Msg>) -> Self {
        self.children.push(child);
        self
    }
    pub fn add_class<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.class_list.push(name.into());
        self
    }

    pub fn href<S>(mut self, href: S) -> Self
    where
        S: Into<String>,
    {
        self.href = href.into();
        self
    }

    pub fn text<S>(mut self, s: S) -> Self
    where
        S: Into<String>,
    {
        let text: String = s.into();
        self.children.push(span![text]);
        self
    }

    pub fn build(self) -> StyledLink {
        StyledLink {
            children: self.children,
            class_list: self.class_list,
            href: self.href,
        }
    }
}

impl ToNode for StyledLink {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

pub fn render(values: StyledLink) -> Node<Msg> {
    let StyledLink {
        children,
        mut class_list,
        href,
    } = values;
    class_list.push("styledLink".to_string());

    a![
        attrs![
            At::Class => class_list.join(" "),
            At::Href => href,
        ],
        children,
    ]
}
