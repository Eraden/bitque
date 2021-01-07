extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};

#[proc_macro_derive(EnumIter)]
pub fn derive_enum_iter(item: TokenStream) -> TokenStream {
    let mut it = item.into_iter().peekable();
    while let Some(token) = it.peek() {
        if let TokenTree::Ident(_) = token {
            break;
        } else {
            it.next();
        }
    }
    if let Some(TokenTree::Ident(ident)) = it.next() {
        if ident.to_string().as_str() != "pub" {
            panic!("Expect to find keyword pub but was found {:?}", ident)
        }
    } else {
        panic!("Expect to find keyword pub but nothing was found")
    }
    if let Some(TokenTree::Ident(ident)) = it.next() {
        if ident.to_string().as_str() != "enum" {
            panic!("Expect to find keyword struct but was found {:?}", ident)
        }
    } else {
        panic!("Expect to find keyword struct but nothing was found")
    }
    let name = it
        .next()
        .expect("Expect to struct name but nothing was found");

    let mut variants = vec![];
    if let Some(TokenTree::Group(group)) = it.next() {
        for token in group.stream() {
            if let TokenTree::Ident(ident) = token {
                variants.push(ident.to_string())
            }
        }
    } else {
        panic!("Enum variants group expected");
    }
    if variants.is_empty() {
        panic!("Enum cannot be empty")
    }

    let mut code = format!(
        r#"
pub struct {name}Iter(Option<{name}>);

impl std::iter::Iterator for {name}Iter {{
    type Item = {name};
    
    fn count(self) -> usize {{
        {len}
    }}
    
    fn last(self) -> Option<Self::Item> {{
        Some({name}::{last})
    }}

    fn next(&mut self) -> Option<Self::Item> {{
        let o = match self.0 {{
"#,
        name = name,
        len = variants.len(),
        last = variants.last().unwrap(),
    );

    let mut last_variant = "";
    for (idx, variant) in variants.iter().enumerate() {
        match idx {
            0 => code.push_str(
                format!(
                    "None => Some({name}::{variant}),\n",
                    variant = variant,
                    name = name
                )
                .as_str(),
            ),
            _ if idx == variants.len() - 1 => code.push_str("_ => None,\n"),
            _ => code.push_str(
                format!(
                    "Some({name}::{last_variant}) => Some({name}::{variant}),\n",
                    last_variant = last_variant,
                    variant = variant,
                    name = name,
                )
                .as_str(),
            ),
        }
        last_variant = variant.as_str();
    }

    code.push_str(
        format!(
            r#"
        }};
        self.0 = o;
        o
    }}
}}
impl std::iter::IntoIterator for {name} {{
    type Item = {name};
    type IntoIter = {name}Iter;

    fn into_iter(self) -> Self::IntoIter {{
        {name}Iter(None)
    }}
}}
"#,
            name = name
        )
        .as_str(),
    );
    code.parse().unwrap()
}
