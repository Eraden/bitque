
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
      regex: Regex::new("(?i:(\\!|^[Cc])\\$omp\\b)"),
      scope: vec![
        Scope {
            a: 61925255092832453,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\&(?m:$)"),
      scope: vec![
        Scope {
            a: 47297447026032640,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?m:$)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\!.*"),
      scope: vec![
        Scope {
            a: 51510711159160832,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7436 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7428 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7437 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7439 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7441 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7438 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7427 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7429 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7440 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7433 })),
]
} }