
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
      regex: Regex::new("([A-Z0-9_\\-]+)(\\()([^)]+)(\\))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466373918720,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628098744320,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 55451420830269440,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636628098744320,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?m:$)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8335 }),
    ]),
      with_prototype: None
    }),
]
} }