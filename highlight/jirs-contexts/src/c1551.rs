
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
        index: 1627,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\?{,2}(?:[_A-Za-z][_A-Za-z\\d@]*|\\\'((?:\\\\\\\\)*\\\\\\\'|[^\'\'])*\\\')\\s*\\()"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1558 }),
        ContextReference::Direct(ContextId { index: 1552 }),
        ContextReference::Direct(ContextId { index: 1561 }),
        ContextReference::Direct(ContextId { index: 1533 }),
    ]),
      with_prototype: None
    }),
]
} }