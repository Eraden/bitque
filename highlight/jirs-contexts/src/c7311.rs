
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
      regex: Regex::new("(?xi)\n(?:\n  \\b  (call)\n  \\s+ (?: ([A-Za-z_][A-Za-z_0-9]*) \\s* % \\s* ([A-Za-z_][A-Za-z_0-9]*)\n        | ([A-Za-z_][A-Za-z_0-9]*)\n      )\n)\n"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636877353975808,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087439134720,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925255157907456,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 61925255157907456,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }