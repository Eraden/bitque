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
    placeholder: Option<String>,
    disable_auto_resize: bool,
}

impl ToNode for StyledTextarea {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

impl StyledTextarea {
    pub fn build(field_id: FieldId) -> StyledTextareaBuilder {
        StyledTextareaBuilder {
            id: field_id,
            height: None,
            max_height: None,
            on_change: None,
            value: "".to_string(),
            class_list: vec![],
            update_event: None,
            placeholder: None,
            disable_auto_resize: false,
        }
    }
}

#[derive(Debug)]
pub struct StyledTextareaBuilder {
    id: FieldId,
    height: Option<usize>,
    max_height: Option<usize>,
    on_change: Option<EventHandler<Msg>>,
    value: String,
    class_list: Vec<String>,
    update_event: Option<Ev>,
    placeholder: Option<String>,
    disable_auto_resize: bool,
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

    pub fn placeholder<S>(mut self, placeholder: S) -> Self
    where
        S: Into<String>,
    {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn disable_auto_resize(mut self) -> Self {
        self.disable_auto_resize = true;
        self
    }

    #[inline]
    pub fn build(self) -> StyledTextarea {
        StyledTextarea {
            id: self.id,
            value: self.value,
            height: self.height.unwrap_or(110),
            class_list: self.class_list,
            max_height: self.max_height.unwrap_or_default(),
            update_event: self.update_event.unwrap_or_else(|| Ev::KeyUp),
            placeholder: self.placeholder,
            disable_auto_resize: self.disable_auto_resize,
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
        placeholder,
        disable_auto_resize,
    } = values;
    let mut style_list = vec![];

    let min_height = get_min_height(value.as_str(), height as f64, disable_auto_resize);
    if min_height > 0f64 {
        style_list.push(format!("min-height: {}px", min_height));
    }

    if max_height > 0 {
        style_list.push(format!("max-height: {}px", max_height));
    }

    if disable_auto_resize {
        style_list.push("resize: none".to_string());
        style_list.push(format!(
            "height: {h}px; max-height: {h}px; min-height: {h}px",
            h = max_height
        ));
    }

    let handler_disable_auto_resize = disable_auto_resize;
    let resize_handler = ev(Ev::KeyUp, move |event| {
        event.stop_propagation();
        if handler_disable_auto_resize {
            return Msg::NoOp;
        }

        let target = event.target().unwrap();
        let textarea = seed::to_textarea(&target);
        let value = textarea.value();
        let min_height = get_min_height(value.as_str(), height as f64, handler_disable_auto_resize);

        textarea
            .style()
            .set_css_text(format!("height: {min_height}px", min_height = min_height).as_str());
        Msg::NoOp
    });

    let handler_disable_auto_resize = disable_auto_resize;
    let text_input_handler = ev(update_event, move |event| {
        event.stop_propagation();

        let target = event.target().unwrap();
        let textarea = seed::to_textarea(&target);
        let value = textarea.value();

        if handler_disable_auto_resize && value.contains('\n') {
            event.prevent_default();
        }

        Msg::InputChanged(
            id,
            if handler_disable_auto_resize {
                value.trim().to_string()
            } else {
                value
            },
        )
    });

    class_list.push("textAreaInput".to_string());

    div![
        attrs![At::Class => "styledTextArea"],
        div![attrs![At::Class => "textAreaHeading"]],
        textarea![
            attrs![
                At::Class => class_list.join(" ");
                At::AutoFocus => "true";
                At::Style => style_list.join(";");
                At::Placeholder => placeholder.unwrap_or_default();
                At::Rows => if disable_auto_resize { "1" } else { "auto" }
            ],
            value,
            resize_handler,
            text_input_handler,
        ]
    ]
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
