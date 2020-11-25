
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
      regex: Regex::new("\\b(scope)\\s*(\\()\\s*(exit|success|failure)\\s*(\\))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636701196304,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288521948004534,
            b: 4503599627370496,
        },
    ]),(3, vec![
        Scope {
            a: 52636636701196304,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288521948004523,
            b: 4503599627370496,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1307 }),
    ]),
      with_prototype: None
    }),
]
} }