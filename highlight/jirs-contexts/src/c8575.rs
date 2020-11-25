
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
        a: 104156212513407107,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 104156212513407107,
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
      regex: Regex::new("\\)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288521944400043,
            b: 36873221949095936,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8599 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8598 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8597 })),
]
} }