extern crate proc_macro;

use {
    proc_macro::{token_stream::IntoIter, TokenStream, TokenTree},
    std::iter::Peekable,
};

fn skip_meta(mut it: Peekable<IntoIter>) -> Peekable<IntoIter> {
    while let Some(token) = it.peek() {
        if let TokenTree::Ident(_) = token {
            break;
        } else {
            it.next();
        }
    }
    it
}

fn consume_ident(mut it: Peekable<IntoIter>, name: &str) -> Peekable<IntoIter> {
    if let Some(TokenTree::Ident(ident)) = it.next() {
        if ident.to_string().as_str() != name {
            panic!("Expect to find keyword {} but was found {:?}", name, ident)
        }
    } else {
        panic!("Expect to find keyword {} but nothing was found", name)
    }
    it
}

pub(in crate) fn codegen(mut it: Peekable<IntoIter>) -> Result<String, String> {
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
        return Err("Enum variants group expected".to_string());
    }
    if variants.is_empty() {
        return Err("Enum cannot be empty".to_string());
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
                    "            None => Some({name}::{variant}),\n",
                    variant = variant,
                    name = name
                )
                .as_str(),
            ),
            _ => code.push_str(
                format!(
                    "            Some({name}::{last_variant}) => Some({name}::{variant}),\n",
                    last_variant = last_variant,
                    variant = variant,
                    name = name,
                )
                .as_str(),
            ),
        }
        if idx == variants.len() - 1 {
            code.push_str("            _ => None,\n");
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
    Ok(code)
}

#[proc_macro_derive(EnumIter)]
pub fn derive_enum_iter(item: TokenStream) -> TokenStream {
    let mut it = item.into_iter().peekable();
    it = skip_meta(it);
    it = consume_ident(it, "pub");
    it = consume_ident(it, "enum");

    let code = codegen(it).unwrap();
    code.parse().unwrap()
}

// #[cfg(test)]
// mod tests {
//     use super::codegen;
//     use proc_macro::TokenStream;
//     use std::str::FromStr;
//
//     #[test]
//     fn empty_enum() {
//         let it = TokenStream::from_str("enum A {}")
//             .unwrap()
//             .into_iter()
//             .peekable();
//         let code = codegen(it);
//         assert_eq!(code, Err("Enum cannot be empty".to_string()));
//     }
// }
