
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
      regex: Regex::new("(?<!=|:)\\s*(?:(\\.\\.\\.)\\s*)?(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628265336982,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629476131646,
            b: 42221246506598400,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9418 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!=|:)\\s*(?:(\\.\\.\\.)\\s*)?(\\[)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628265336982,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629476130997,
            b: 42221246506598400,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9419 }),
    ]),
      with_prototype: None
    }),
]
} }