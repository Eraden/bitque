
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
      regex: Regex::new("(?<=\\))\\s*(:)(?=\\s*([^<>\\(\\)]|\\<[^<>]+\\>|\\([^\\(\\)]+\\))+\\s*=>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445192233287680,
            b: 0,
        },
        Scope {
            a: 46444878704214440,
            b: 42502721483309056,
        },
        Scope {
            a: 52636628112179598,
            b: 42502721483309056,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9802 }),
    ]),
      with_prototype: None
    }),
]
} }