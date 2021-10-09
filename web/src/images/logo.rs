use seed::prelude::*;

use crate::Msg;

static LOGO: &str = include_str!("../../static/logo2.svg");

#[inline(always)]
pub fn render() -> Vec<Node<Msg>> {
    Node::from_html(LOGO)
}
