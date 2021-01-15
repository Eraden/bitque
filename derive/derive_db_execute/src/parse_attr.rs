use proc_macro::{token_stream::IntoIter, TokenTree};
use std::iter::Peekable;

#[derive(Default, Debug)]
pub struct Attributes {
    pub result: Option<String>,
    pub schema: Option<String>,
    pub find: Option<String>,
    pub load: Option<String>,
    pub create: Option<String>,
    pub destroy: Option<String>,
    pub update: Option<String>,
}

pub fn parse(mut it: Peekable<IntoIter>) -> (Peekable<IntoIter>, Option<Attributes>) {
    it.next();
    let group = if let Some(TokenTree::Group(group)) = it.next() {
        group
    } else {
        panic!("Expect meta group");
    };
    let mut git = group.stream().into_iter();
    let ident = if let Some(TokenTree::Ident(ident)) = git.next() {
        ident
    } else {
        panic!("Expect attribute name")
    };
    if ident.to_string().as_str() != "db_exec" {
        return (it, None);
    }

    let group = if let Some(TokenTree::Group(group)) = git.next() {
        group
    } else {
        panic!("Expect attribute name")
    };
    (it, Some(parse_db_exec(group.stream().into_iter())))
}

fn parse_db_exec(mut it: IntoIter) -> Attributes {
    let mut attrs = Attributes::default();
    while let Some(token) = it.next() {
        let ident = if let TokenTree::Ident(ident) = token {
            ident
        } else {
            continue;
        };
        match ident.to_string().as_str() {
            "result" => {
                attrs.result = Some(fetch_name(&mut it));
            }
            "schema" => {
                attrs.schema = Some(fetch_name(&mut it));
            }
            "find" => {
                attrs.find = Some(fetch_name(&mut it));
            }
            "load" => {
                attrs.load = Some(fetch_name(&mut it));
            }
            "create" => {
                attrs.create = Some(fetch_name(&mut it));
            }
            "destroy" => {
                attrs.destroy = Some(fetch_name(&mut it));
            }
            "update" => {
                attrs.update = Some(fetch_name(&mut it));
            }
            _ => continue,
        };
    }
    attrs
}

fn fetch_name(it: &mut IntoIter) -> String {
    if let Some(TokenTree::Punct(_)) = it.next() {
    } else {
        panic!("Expect equal token");
    }
    let lit = if let Some(TokenTree::Literal(lit)) = it.next() {
        lit
    } else {
        panic!("Expect type name as string");
    };
    let mut name = lit.to_string();

    if name.starts_with('"') {
        name.remove(0);
        name.remove(name.len() - 1);
    } else if name.starts_with("r#\"") {
        name.remove(0);
        name.remove(0);
        name.remove(0);
        name.remove(name.len() - 1);
        name.remove(name.len() - 1);
    }
    let name = name.trim();

    name.to_string()
}
