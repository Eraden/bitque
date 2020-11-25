
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
  prototype: Some(
    ContextId {
        index: 5074,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$(?:(?:[\\p{L}][_\\p{L}\\p{N}]*|_[_\\p{L}\\p{N}]+)\\b))(:)(ident|path|expr|ty|pat|stmt|block|item|meta|tt)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49258876868165704,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620725829632,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576467247104,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }