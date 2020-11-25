
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 3977 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\bNS\\w+\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3930 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 665 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 745 })),
]
} }