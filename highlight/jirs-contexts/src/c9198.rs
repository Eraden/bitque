
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
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629327560875,
            b: 40813871623045120,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9229 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9213 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9216 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9222 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9231 })),
]
} }