
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
        index: 1314,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bclass\\b"),
      scope: vec![
        Scope {
            a: 46444243036667904,
            b: 0,
        },
        Scope {
            a: 48414576475832336,
            b: 0,
        },
        Scope {
            a: 52638212954980368,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1283 }),
        ContextReference::Direct(ContextId { index: 1299 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\.?\\s*[\\p{L}_][\\p{L}0-9_]*(\\s*\\.\\s*[\\p{L}_][\\p{L}0-9_]*)*\\s*\\()"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1296 }),
        ContextReference::Direct(ContextId { index: 1301 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1296 }),
        ContextReference::Direct(ContextId { index: 1335 }),
    ]),
      with_prototype: None
    }),
]
} }