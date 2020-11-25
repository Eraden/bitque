
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
      regex: Regex::new("^\\s*(import)\\s+[^ $]+\\s+(as)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787168378880,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787168378880,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }