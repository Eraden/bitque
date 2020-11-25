
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
        a: 46446218721951744,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46446218721951744,
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
      regex: Regex::new("(?!\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\\')(?:(t?format)(:))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956406,
            b: 20829148276588544,
        },
    ]),(2, vec![
        Scope {
            a: 52638212985126933,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636628111131290,
            b: 186617999753478144,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1731 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\"(?:(t?format)(:))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956406,
            b: 20829148276588544,
        },
    ]),(2, vec![
        Scope {
            a: 52638212985126933,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1732 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1759 })),
]
} }