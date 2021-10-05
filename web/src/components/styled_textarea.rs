use seed::prelude::*;
use seed::*;

use crate::{FieldId, Msg};

#[derive(Debug)]
pub struct StyledTextarea<'l> {
    pub id: Option<FieldId>,
    pub height: usize,
    pub max_height: usize,
    pub value: &'l str,
    pub class_list: &'l str,
    pub update_event: Ev,
    pub placeholder: &'l str,
    pub disable_auto_resize: bool,
}

impl<'l> Default for StyledTextarea<'l> {
    fn default() -> Self {
        Self {
            id: None,
            height: 0,
            max_height: 0,
            value: "",
            class_list: "",
            update_event: Ev::Change,
            placeholder: "",
            disable_auto_resize: false,
        }
    }
}

impl<'l> StyledTextarea<'l> {
    // height = `calc( (${$0.value.split("\n").length}px * ( 15 * 1.4285 )) + 17px +
    // 2px)` where:
    //  * 15 is font-size
    //  * 1.4285 is line-height
    //  * 17 is padding top + bottom
    //  * 2 is border top + bottom
    pub fn render(self) -> Node<Msg> {
        let StyledTextarea {
            id,
            height,
            max_height,
            value,
            class_list,
            update_event,
            placeholder,
            disable_auto_resize,
        } = self;
        let id = id.expect("Text area requires FieldId");
        let mut style_list = vec![];

        let min_height = get_min_height(value, height as f64, disable_auto_resize);
        if min_height > 0f64 {
            style_list.push(format!("min-height: {}px", min_height));
        }

        if max_height > 0 {
            style_list.push(format!("max-height: {}px", max_height));
        }

        if disable_auto_resize {
            style_list.push(format!(
                "resize: none; height: {h}px; max-height: {h}px; min-height: {h}px",
                h = max_height
            ));
        }

        // let handler_disable_auto_resize = disable_auto_resize;
        // let resize_handler = ev(Ev::KeyUp, move |event| {
        //     event.stop_propagation();
        //     if handler_disable_auto_resize {
        //         return None as Option<Msg>;
        //     }
        //
        //     let target = event.target().unwrap();
        //     let textarea = seed::to_textarea(&target);
        //     let value = textarea.value();
        //     let min_height =
        //         get_min_height(value.as_str(), height as f64,
        // handler_disable_auto_resize);
        //
        //     textarea
        //         .style()
        //         .set_css_text(format!("height: {min_height}px", min_height =
        // min_height).as_str());     None as Option<Msg>
        // });

        let handler_disable_auto_resize = disable_auto_resize;
        let text_input_handler = {
            let id = id.clone();
            ev(update_event, move |event| {
                event.stop_propagation();

                let value = event
                    .target()
                    .map(|target| seed::to_textarea(&target).value())
                    .unwrap_or_default();

                if handler_disable_auto_resize && value.contains('\n') {
                    event.prevent_default();
                }

                Some(Msg::StrInputChanged(
                    id,
                    if handler_disable_auto_resize {
                        value.trim().to_string()
                    } else {
                        value
                    },
                ))
            })
        };

        div![
            id![format!("styledTextArea-{}", id)],
            C!["styledTextArea"],
            div![C!["textAreaHeading"]],
            textarea![
                C![class_list, "textAreaInput"],
                attrs![
                    At::AutoFocus => "true";
                    At::Style => style_list.join(";");
                    At::Placeholder => placeholder;
                    At::Rows => if disable_auto_resize { "5" } else { "auto" },
                    At::Data => height
                ],
                value,
                // resize_handler,
                text_input_handler,
            ]
        ]
    }
}

const FONT_SIZE: f64 = 15f64;
const LINE_HEIGHT: f64 = 1.4285;
const LETTER_HEIGHT: f64 = FONT_SIZE * LINE_HEIGHT;
const PADDING_TOP_BOTTOM: f64 = 17f64;
const BORDER_TOP_BOTTOM: f64 = 2f64;
const ADDITIONAL_HEIGHT: f64 = PADDING_TOP_BOTTOM + BORDER_TOP_BOTTOM;

pub fn handle_resize(target: &web_sys::Element) -> Option<Msg> {
    let height: usize = target.get_attribute("data")?.parse().ok()?;
    let textarea = seed::to_textarea(target);
    let value = textarea.value();
    let min_height = get_min_height(value.as_str(), height as f64, false);

    textarea
        .style()
        .set_css_text(format!("height: {min_height}px", min_height = min_height).as_str());
    None
}

fn get_min_height(value: &str, min_height: f64, disable_auto_resize: bool) -> f64 {
    if disable_auto_resize {
        return min_height;
    }
    let len = value.lines().count() as f64;
    // if value.chars().last() == Some('\n') {}

    let calc_height = (len * LETTER_HEIGHT) + ADDITIONAL_HEIGHT;
    if calc_height + ADDITIONAL_HEIGHT < min_height {
        min_height
    } else {
        calc_height + ADDITIONAL_HEIGHT
    }
}
