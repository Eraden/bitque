
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
        a: 46444217269878784,
        b: 0,
    },
    Scope {
        a: 55451420831973599,
        b: 17451448556060672,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444217269878784,
        b: 0,
    },
    Scope {
        a: 55451420831973599,
        b: 17451448556060672,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\s*(?:SELECT|INSERT|UPDATE|DELETE|CREATE|REPLACE|ALTER|WITH)\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4518 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4520 }),
    ]),
      with_prototype: None
    }),
]
} }