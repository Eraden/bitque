
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 997 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1014 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1023 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1020 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1012 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1042 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1015 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(auto-fill|auto-fit)\\b"),
      scope: vec![
        Scope {
            a: 61925298067931150,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(max-content|min-content|auto)\\b"),
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