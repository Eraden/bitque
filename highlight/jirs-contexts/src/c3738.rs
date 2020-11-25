
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
        a: 46444131370139648,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444131370139648,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 3708 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\()"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3600 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }