
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
        a: 46453309858709504,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46453309858709504,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 7370 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([A-Za-z_][A-Za-z_0-9]*)"),
      scope: vec![
        Scope {
            a: 49258876990261445,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }