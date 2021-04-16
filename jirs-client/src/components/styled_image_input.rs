use seed::prelude::*;
use seed::*;
use web_sys::File;

use crate::shared::ToNode;
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

impl<'l> ToNode for StyledImageInput<'l> {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

fn render(values: StyledImageInput) -> Node<Msg> {
    let StyledImageInput {
        id,
        class_list,
        url,
    } = values;

    let field_id = id.clone();
    let on_change = ev(Ev::Change, move |ev| {
        let target = ev.target().unwrap();
        let input = seed::to_input(&target);
        let v = input
            .files()
            .map(|list| {
                let mut v = vec![];
                for i in 0..list.length() {
                    v.push(list.get(i).unwrap());
                }
                v
            })
            .unwrap_or_default();
        Msg::FileInputChanged(field_id, v)
    });
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
