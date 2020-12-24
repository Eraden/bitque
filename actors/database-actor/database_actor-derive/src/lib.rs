extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};

#[proc_macro_derive(DbMsg, attributes(query))]
pub fn db_msg(item: TokenStream) -> TokenStream {
    let mut it = item.into_iter();
    if let Some(TokenTree::Ident(ident)) = it.next() {
        if ident.to_string().as_str() != "pub" {
            panic!("Expect to find keyword pub but was found {:?}", ident)
        }
    } else {
        panic!("Expect to find keyword pub but nothing was found")
    }
    if let Some(TokenTree::Ident(ident)) = it.next() {
        if ident.to_string().as_str() != "struct" {
            panic!("Expect to find keyword struct but was found {:?}", ident)
        }
    } else {
        panic!("Expect to find keyword struct but nothing was found")
    }
    let _name = it
        .next()
        .expect("Expect to struct name but nothing was found");

    "".parse().unwrap()
}
