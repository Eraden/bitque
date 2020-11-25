
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
        a: 844613915181056,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844613915181056,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 6930 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=.|\\n)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6919 }),
    ]),
      with_prototype: None
    }),
]
} }