
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 46446618168524805,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46446618168524805,
        b: 0,
    },
],
  meta_include_prototype: false,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\""),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 55451420828565504,
            b: 0,
        },
        Scope {
            a: 47288629323956395,
            b: 1407374883553280,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }