use seed::*;
use seed::prelude::*;
use web_sys::File;

use crate::{FieldId, Msg};

#[derive(Debug, Clone)]
pub struct StyledImageInputState {
    field_id: FieldId,
    pub file: Option<File>,
    pub url: Option<String>,
}

impl StyledImageInputState {
    pub fn new(field_id: FieldId, url: Option<String>) -> Self {
        Self {
            field_id,
            file: None,
            url,
        }
    }

    pub fn update(&mut self, msg: &Msg) {
        match msg {
            Msg::FileInputChanged(field_id, files) if field_id == &self.field_id => {
                self.file = files.get(0).cloned();
            }
            _ => {}
        }
    }
}

pub struct StyledImageInput<'l> {
    pub id: FieldId,
    pub class_list: &'l str,
    pub url: Option<&'l str>,
}

impl<'l> StyledImageInput<'l> {
    #[inline(always)]
    pub fn render(self) -> Node<Msg> {
        let StyledImageInput {
            id,
            class_list,
            url,
        } = self;

        let on_change = super::events::on_change_image_input(id.clone());
        let input_id = id.to_string();

        div![
            C!["styledImageInput", class_list],
            label![
                C!["label"],
                attrs![At::For => input_id],
                img![C!["mask"], attrs![At::Src => url.unwrap_or_default()], " "]
            ],
            input![
                C!["input"],
                attrs![At::Type => "file", At::Id => input_id],
                on_change
            ]
        ]
    }
}
