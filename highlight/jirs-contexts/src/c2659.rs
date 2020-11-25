
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
        index: 2664,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2612 }),
        ContextReference::Direct(ContextId { index: 2521 }),
        ContextReference::Direct(ContextId { index: 2607 }),
        ContextReference::Direct(ContextId { index: 2520 }),
        ContextReference::Direct(ContextId { index: 2522 }),
        ContextReference::Direct(ContextId { index: 2599 }),
    ]),
      with_prototype: None
    }),
]
} }