use {
    crate::{shared::ToNode, FieldId, Msg},
    seed::{prelude::*, *},
    web_sys::File,
};

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
    id: FieldId,
    class_list: Vec<&'l str>,
    url: Option<String>,
}

impl<'l> StyledImageInput<'l> {
    pub fn build(field_id: FieldId) -> StyledInputInputBuilder<'l> {
        StyledInputInputBuilder {
            id: field_id,
            class_list: vec![],
            url: None,
        }
    }
}

impl<'l> ToNode for StyledImageInput<'l> {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

pub struct StyledInputInputBuilder<'l> {
    id: FieldId,
    class_list: Vec<&'l str>,
    url: Option<String>,
}

impl<'l> StyledInputInputBuilder<'l> {
    pub fn add_class(mut self, name: &'l str) -> Self {
        self.class_list.push(name);
        self
    }

    pub fn state(mut self, state: &StyledImageInputState) -> Self {
        self.url = state.url.as_ref().cloned();
        self
    }

    pub fn build(self) -> StyledImageInput<'l> {
        StyledImageInput {
            id: self.id,
            class_list: self.class_list,
            url: self.url,
        }
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
        C!["styledImageInput"],
        attrs![At::Class => class_list.join(" ")],
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
