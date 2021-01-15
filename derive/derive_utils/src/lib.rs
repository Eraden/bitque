use proc_macro::{token_stream::IntoIter, TokenTree};
use std::iter::Peekable;

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

pub fn skip_enum(mut it: Peekable<IntoIter>) -> Peekable<IntoIter> {
    if let Some(TokenTree::Ident(ident)) = it.next() {
        if ident.to_string().as_str() != "enum" {
            panic!("Expect to find keyword enum but was found {:?}", ident)
        }
    } else {
        panic!("Expect to find keyword enum but nothing was found")
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
