
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
      regex: Regex::new("\"(?!\")"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956395,
            b: 36873221949095936,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\\.[A-Z]{2,64}\\b"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8605 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8594 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8598 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("`\\s*(?m:$)"),
      scope: vec![
        Scope {
            a: 52636787021119488,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }