
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
        a: 845060585291776,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 845060585291776,
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
      regex: Regex::new("^[ \\t]*"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9316 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9343 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9328 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9320 })),
]
} }