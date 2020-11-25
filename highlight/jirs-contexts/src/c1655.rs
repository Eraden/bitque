
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
        index: 1627,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("fun(?=[^[_A-Za-z\\d@]])"),
      scope: vec![
        Scope {
            a: 61925375345950720,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1656 }),
        ContextReference::Direct(ContextId { index: 1657 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\?{,2}(?:[_A-Za-z][_A-Za-z\\d@]*|\\\'((?:\\\\\\\\)*\\\\\\\'|[^\'\'])*\\\')\\s*\\()"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1658 }),
        ContextReference::Direct(ContextId { index: 1657 }),
        ContextReference::Direct(ContextId { index: 1643 }),
    ]),
      with_prototype: None
    }),
]
} }