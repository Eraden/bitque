
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![],
  meta_content_scope: vec![],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\\\\\n)|(\\\\\\\\)|(\\\\\\\")|(\\\\\')|(\\\\a)|(\\\\b)|(\\\\f)|(\\\\n)|(\\\\r)|(\\\\t)|(\\\\v)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955200847315291,
            b: 33495522228568064,
        },
    ]),(2, vec![
        Scope {
            a: 59955200847317308,
            b: 33495522228568064,
        },
    ]),(3, vec![
        Scope {
            a: 59955200847317309,
            b: 33495522228568064,
        },
    ]),(4, vec![
        Scope {
            a: 59955200847317310,
            b: 33495522228568064,
        },
    ]),(5, vec![
        Scope {
            a: 59955200847317311,
            b: 33495522228568064,
        },
    ]),(6, vec![
        Scope {
            a: 59955200847317130,
            b: 33495522228568064,
        },
    ]),(7, vec![
        Scope {
            a: 59955200847317312,
            b: 33495522228568064,
        },
    ]),(8, vec![
        Scope {
            a: 59955200847317313,
            b: 33495522228568064,
        },
    ]),(9, vec![
        Scope {
            a: 59955200847315295,
            b: 33495522228568064,
        },
    ]),(10, vec![
        Scope {
            a: 59955200847315874,
            b: 33495522228568064,
        },
    ]),(11, vec![
        Scope {
            a: 59955200847317314,
            b: 33495522228568064,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }