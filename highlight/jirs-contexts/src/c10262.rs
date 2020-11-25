
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
  meta_include_prototype: false,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\\\)\\\"(?=\\s*(?:#.*)?)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 55451420828565665,
            b: 0,
        },
        Scope {
            a: 47288629323956395,
            b: 45317471250415616,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }