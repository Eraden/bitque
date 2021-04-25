use seed::prelude::*;
use seed::*;

use crate::model::Model;
use crate::{BuildMsg, Msg};

pub fn styled_tip<B>(letter: char, model: &Model, builder: B) -> Node<Msg>
where
    B: BuildMsg + 'static,
{
    model
        .key_triggers
        .borrow_mut()
        .insert(letter, Box::new(builder));
    div![
        C!["proTip"],
        strong![C!["strong"], "Pro tip: "],
        "press ",
        span![C!["tipLetter", letter.to_string()], letter.to_string()],
        " to comment"
    ]
}
