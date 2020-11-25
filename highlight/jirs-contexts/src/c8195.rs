
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
      regex: Regex::new("(?=\\))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8304 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8298 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8264 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8276 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8275 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8312 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8296 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8297 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bto\\b"),
      scope: vec![
        Scope {
            a: 52636787013451776,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(top|right|bottom|left)\\b"),
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
]
} }