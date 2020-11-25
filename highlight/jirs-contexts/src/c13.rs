
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
  clear_scopes: Some(
    ClearAmount::All,
),
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<%=?"),
      scope: vec![
        Scope {
            a: 47288521949642934,
            b: 55169112615157760,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s+(?m:$)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 14 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\s*\")?(?=[^<>]*>|\\s+\\w+=\\s*\")"),
      scope: vec![
        Scope {
            a: 55451420828565509,
            b: 0,
        },
        Scope {
            a: 47288629323956395,
            b: 1407374883553280,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 15 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 16 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 92 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 17 }),
    )
    }),
]
} }