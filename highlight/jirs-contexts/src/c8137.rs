
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 8277 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(nonzero|evenodd)\\b"),
      scope: vec![
        Scope {
            a: 61925409737015310,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8296 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8312 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8271 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8276 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8304 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8298 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8297 })),
]
} }