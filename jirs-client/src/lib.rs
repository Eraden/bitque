use seed::{prelude::*, *};

mod model;

type Model = i32;

enum Msg {
    Increment,
    ChangeGuidePage(i32),
    ChangePage(i32),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        class!["counter"],
        button![model.to_string(), ev(Ev::Click, |_| Msg::Increment),],
    ]
}

fn routes(url: Url) -> Option<Msg> {
    if url.path.is_empty() {
        return Some(Msg::ChangePage(0));
    }

    match url.path[0].as_ref() {
        "guide" => match url.path.get(1).as_ref() {
            Some(page) => Some(Msg::ChangeGuidePage(page.parse::<usize>().unwrap())),
            None => Some(Msg::ChangePage(0)),
        },
        "changelog" => Some(Msg::ChangePage(1)),
        _ => Some(Msg::ChangePage(0)),
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).routes(routes).build_and_start();
}
