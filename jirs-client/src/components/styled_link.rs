use {
    crate::{shared::ToNode, Msg},
    seed::{prelude::*, *},
    std::str::FromStr,
};

pub struct StyledLink<'l> {
    pub children: Vec<Node<Msg>>,
    pub class_list: &'l str,
    pub href: &'l str,
}

impl<'l> StyledLink<'l> {
    // pub fn build() -> StyledLinkBuilder<'l> {
    //     StyledLinkBuilder::default()
    // }
}

#[derive(Default)]
pub struct StyledLinkBuilder<'l> {
    children: Vec<Node<Msg>>,
    class_list: &'l str,
    href: &'l str,
}

impl<'l> StyledLinkBuilder<'l> {
    pub fn add_child(mut self, child: Node<Msg>) -> Self {
        self.children.push(child);
        self
    }

    pub fn class_list(mut self, name: &'l str) -> Self {
        self.class_list = name;
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

    let on_click = {
        let href = href.to_string();
        mouse_ev("click", move |ev| {
            if href.starts_with('/') {
                ev.prevent_default();
                ev.stop_propagation();
                if let Ok(url) = seed::Url::from_str(href.as_str()) {
                    url.go_and_push();
                }
            }

            None as Option<Msg>
        })
    };

    a![
        C!["styledLink", class_list],
        attrs![ At::Href => href, ],
        on_click,
        children,
    ]
}
