
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
        a: 46453142354984960,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46453142354984960,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 7370 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b print \\b)"),
      scope: vec![
        Scope {
            a: 61925255134447813,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*"),
      scope: vec![
        Scope {
            a: 47290532030449861,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7338 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7336 })),
]
} }