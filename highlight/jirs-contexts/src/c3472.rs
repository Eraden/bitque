
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
      regex: Regex::new("(#[a-z_][a-zA-Z0-9\'_]*)|(int|char|float|string|list|array|bool|unit|exn|option|int32|int64|nativeint|format4|lazy_t)|([a-z_][a-zA-Z0-9\'_]*)\\s*(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576530030644,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576465936384,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576465936384,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }