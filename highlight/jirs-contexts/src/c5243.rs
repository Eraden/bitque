
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
        index: 5277,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(for)\\s*(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636701196361,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288521951477942,
            b: 20547673299877888,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5146 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(for)\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636701196361,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288521944400054,
            b: 20547673299877888,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5147 }),
    ]),
      with_prototype: None
    }),
]
} }