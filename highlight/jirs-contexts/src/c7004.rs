
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
      regex: Regex::new("(?:\\b|\\B-)\\d+(\\.)\\d+(?=[\\s;\"])"),
      scope: vec![
        Scope {
            a: 59955089176592488,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397992,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\b|\\B-)\\d+(?=[\\s;\"])"),
      scope: vec![
        Scope {
            a: 59955089176461416,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6991 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7016 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[^\\s;,]+"),
      scope: vec![
        Scope {
            a: 55451949103316992,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }