
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
  prototype: Some(
    ContextId {
        index: 7635,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("query(?!(?:[_0-9A-Za-z]))"),
      scope: vec![
        Scope {
            a: 52638212980605042,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7639 }),
        ContextReference::Direct(ContextId { index: 7621 }),
        ContextReference::Direct(ContextId { index: 7652 }),
        ContextReference::Direct(ContextId { index: 7636 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\{)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7639 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("mutation(?!(?:[_0-9A-Za-z]))"),
      scope: vec![
        Scope {
            a: 52638213094834290,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7639 }),
        ContextReference::Direct(ContextId { index: 7621 }),
        ContextReference::Direct(ContextId { index: 7652 }),
        ContextReference::Direct(ContextId { index: 7634 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("subscription(?!(?:[_0-9A-Za-z]))"),
      scope: vec![
        Scope {
            a: 52638213094899826,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7639 }),
        ContextReference::Direct(ContextId { index: 7621 }),
        ContextReference::Direct(ContextId { index: 7652 }),
        ContextReference::Direct(ContextId { index: 7640 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("fragment(?!(?:[_0-9A-Za-z]))"),
      scope: vec![
        Scope {
            a: 52638213094965362,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7639 }),
        ContextReference::Direct(ContextId { index: 7621 }),
        ContextReference::Direct(ContextId { index: 7642 }),
        ContextReference::Direct(ContextId { index: 7627 }),
    ]),
      with_prototype: None
    }),
]
} }