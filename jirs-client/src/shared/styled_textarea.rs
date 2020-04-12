use seed::{prelude::*, *};

use crate::shared::ToNode;
use crate::{FieldId, Msg};

#[derive(Debug)]
pub struct StyledTextarea {
    id: FieldId,
    height: usize,
    max_height: usize,
    value: String,
    class_list: Vec<String>,
    update_event: Ev,
}

impl ToNode for StyledTextarea {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

impl StyledTextarea {
    pub fn build() -> StyledTextareaBuilder {
        StyledTextareaBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct StyledTextareaBuilder {
    height: Option<usize>,
    max_height: Option<usize>,
    on_change: Option<EventHandler<Msg>>,
    value: String,
    class_list: Vec<String>,
    update_event: Option<Ev>,
}

impl StyledTextareaBuilder {
    #[inline]
    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    #[inline]
    pub fn max_height(mut self, height: usize) -> Self {
        self.max_height = Some(height);
        self
    }

    #[inline]
    pub fn value<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.value = value.into();
        self
    }

    #[inline]
    pub fn add_class<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.class_list.push(value.into());
        self
    }

    pub fn update_on(mut self, ev: Ev) -> Self {
        self.update_event = Some(ev);
        self
    }

    #[inline]
    pub fn build(self, id: FieldId) -> StyledTextarea {
        StyledTextarea {
            id,
            value: self.value,
            height: self.height.unwrap_or(110),
            class_list: self.class_list,
            max_height: self.max_height.unwrap_or_default(),
            update_event: self.update_event.unwrap_or_else(|| Ev::KeyUp),
        }
    }
}

const FONT_SIZE: f64 = 15f64;
const LINE_HEIGHT: f64 = 1.4285;
const LETTER_HEIGHT: f64 = FONT_SIZE * LINE_HEIGHT;
const PADDING_TOP_BOTTOM: f64 = 17f64;
const BORDER_TOP_BOTTOM: f64 = 2f64;
const ADDITIONAL_HEIGHT: f64 = PADDING_TOP_BOTTOM + BORDER_TOP_BOTTOM;

// height = `calc( (${$0.value.split("\n").length}px * ( 15 * 1.4285 )) + 17px + 2px)`
// where:
//  * 15 is font-size
//  * 1.4285 is line-height
//  * 17 is padding top + bottom
//  * 2 is border top + bottom
pub fn render(values: StyledTextarea) -> Node<Msg> {
    let StyledTextarea {
        id,
        height,
        max_height,
        value,
        mut class_list,
        update_event,
    } = values;
    let mut style_list = vec![];

    let min_height = get_min_height(value.as_str(), height as f64);
    if min_height > 0f64 {
        style_list.push(format!("min-height: {}px", min_height));
    }

    if max_height > 0 {
        style_list.push(format!("max-height: {}px", max_height));
    }

    let mut handlers = vec![];

    let resize_handler = ev(Ev::KeyUp, move |event| {
        use wasm_bindgen::JsCast;

        let target = match event.target() {
            Some(el) => el,
            _ => return Msg::NoOp,
        };
        let text_area = target.dyn_ref::<web_sys::HtmlTextAreaElement>().unwrap();
        let value: String = text_area.value();
        let min_height = get_min_height(value.as_str(), height as f64);

        text_area
            .style()
            .set_css_text(format!("height: {min_height}px", min_height = min_height).as_str());
        Msg::NoOp
    });
    handlers.push(resize_handler);
    let text_input_handler = input_ev(update_event.clone(), move |value| {
        Msg::InputChanged(id, value)
    });
    handlers.push(text_input_handler);
    handlers.push(keyboard_ev(Ev::KeyUp, |ev| {
        ev.stop_propagation();
        Msg::NoOp
    }));

    class_list.push("textAreaInput".to_string());

    div![
        attrs![At::Class => "styledTextArea"],
        div![attrs![At::Class => "textAreaHeading"]],
        textarea![
            attrs![
                At::Class => class_list.join(" ");
                At::AutoFocus => "true";
                At::Style => style_list.join(";");
            ],
            value,
            handlers,
        ]
    ]
}

fn get_min_height(value: &str, min_height: f64) -> f64 {
    let len = value.lines().count() as f64;
    if value.chars().last() == Some('\n') {}

    let calc_height = (len * LETTER_HEIGHT) + ADDITIONAL_HEIGHT;
    if calc_height + ADDITIONAL_HEIGHT < min_height {
        min_height
    } else {
        calc_height + ADDITIONAL_HEIGHT
    }
}
