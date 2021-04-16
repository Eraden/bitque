use std::iter::Peekable;

use proc_macro::token_stream::IntoIter;
use proc_macro::TokenTree;

pub fn skip_pub(mut it: Peekable<IntoIter>) -> Peekable<IntoIter> {
    if let Some(TokenTree::Ident(ident)) = it.next() {
        if ident.to_string().as_str() != "pub" {
            panic!("Expect to find keyword pub but was found {:?}", ident)
        }
    } else {
        panic!("Expect to find keyword pub but nothing was found")
    }
    it
}

pub fn skip_struct(mut it: Peekable<IntoIter>) -> Peekable<IntoIter> {
    if let Some(TokenTree::Ident(ident)) = it.next() {
        if ident.to_string().as_str() != "struct" {
            panic!("Expect to find keyword struct but was found {:?}", ident)
        }
    } else {
        panic!("Expect to find keyword struct but nothing was found")
    }
    it
}
