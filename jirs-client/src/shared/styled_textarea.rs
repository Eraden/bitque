use crate::shared::ToNode;
use crate::{FieldId, Msg};
use seed::{prelude::*, *};

#[derive(Debug)]
pub struct StyledTextarea {
    id: FieldId,
    height: usize,
    on_change: Option<EventHandler<Msg>>,
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
    on_change: Option<EventHandler<Msg>>,
}

impl StyledTextareaBuilder {
    pub fn height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    pub fn on_change(mut self, on_change: EventHandler<Msg>) -> Self {
        self.on_change = Some(on_change);
        self
    }

    pub fn build(self, id: FieldId) -> StyledTextarea {
        StyledTextarea {
            id,
            height: self.height.unwrap_or(110),
            on_change: self.on_change,
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
        on_change,
    } = values;
    let mut style_list = vec![];
    style_list.push(format!("min-height: {}px", height));

    let mut handlers = vec![];
    if let Some(handler) = on_change {
        handlers.push(handler);
    }

    let resize_handler = ev(Ev::KeyPress, move |event| {
        use wasm_bindgen::JsCast;

        let target = match event.target() {
            Some(el) => el,
            _ => return Msg::NoOp,
        };
        let text_area = target.dyn_ref::<web_sys::HtmlTextAreaElement>().unwrap();
        let value: String = text_area.value();
        let len = value.lines().count() as f64;

        let calc_height = (len * LETTER_HEIGHT) + ADDITIONAL_HEIGHT;
        let height = if calc_height + ADDITIONAL_HEIGHT < height as f64 {
            height as f64
        } else {
            calc_height + ADDITIONAL_HEIGHT
        };

        text_area
            .style()
            .set_css_text(format!("height: {height}px", height = height).as_str());
        Msg::NoOp
    });
    handlers.push(resize_handler);
    let text_input_handler = input_ev(Ev::KeyPress, move |value| Msg::InputChanged(id, value));
    handlers.push(text_input_handler);

    div![
        attrs![At::Class => "styledTextArea"],
        div![attrs![At::Class => "textAreaHeading"]],
        textarea![
            attrs![
                At::Class => "textAreaInput";
                At::ContentEditable => "true";
                At::Style => style_list.join(";");
            ],
            handlers,
        ]
    ]
}
