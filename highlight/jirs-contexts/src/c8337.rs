
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
        a: 844957506076672,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844957506076672,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8336 }),
    ]),
      with_prototype: None
    }),
]
} }