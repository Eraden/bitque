
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
        a: 55451949170753597,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 55451949170753597,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: true,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: true,
      regex: Regex::new("^\\3(?m:$)"),
      scope: vec![
        Scope {
            a: 46444230224969789,
            b: 0,
        },
        Scope {
            a: 59392130632123501,
            b: 563211946426368,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }