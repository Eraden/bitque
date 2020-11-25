
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
        a: 55451420830269670,
        b: 381961968598122496,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 55451420830269670,
        b: 381961968598122496,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: true,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: true,
      regex: Regex::new("\\1"),
      scope: vec![
        Scope {
            a: 55451420830269670,
            b: 381961968598122496,
        },
        Scope {
            a: 47288629323956395,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }