use seed::{prelude::*, *};

use crate::shared::ToNode;
use crate::Msg;

pub struct StyledLink<'l> {
    children: Vec<Node<Msg>>,
    class_list: Vec<&'l str>,
    href: &'l str,
}

impl<'l> StyledLink<'l> {
    pub fn build() -> StyledLinkBuilder<'l> {
        StyledLinkBuilder::default()
    }
}

#[derive(Default)]
pub struct StyledLinkBuilder<'l> {
    children: Vec<Node<Msg>>,
    class_list: Vec<&'l str>,
    href: &'l str,
}

impl<'l> StyledLinkBuilder<'l> {
    pub fn add_child(mut self, child: Node<Msg>) -> Self {
        self.children.push(child);
        self
    }

    pub fn add_class(mut self, name: &'l str) -> Self {
        self.class_list.push(name);
        self
    }

    pub fn href(mut self, href: &'l str) -> Self {
        self.href = href;
        self
    }

    pub fn text(self, s: &'l str) -> Self {
        self.add_child(span![s])
    }

    pub fn build(self) -> StyledLink<'l> {
        StyledLink {
            children: self.children,
            class_list: self.class_list,
            href: self.href,
        }
    }
}

impl<'l> ToNode for StyledLink<'l> {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

pub fn render(values: StyledLink) -> Node<Msg> {
    let StyledLink {
        children,
        class_list,
        href,
    } = values;

    a![
        C!["styledLink"],
        attrs![
            At::Class => class_list.join(" "),
            At::Href => href,
        ],
        children,
    ]
}
