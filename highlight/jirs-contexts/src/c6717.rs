
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
      regex: Regex::new("^(?:\\s*)((@)([a-zA-Z0-9_]+))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 112027453047701504,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615438,
            b: 27021597764222976,
        },
    ]),(3, vec![
        Scope {
            a: 61925375350931456,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }