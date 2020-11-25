
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
      regex: Regex::new("\\s*(\\()"),
      scope: vec![
        Scope {
            a: 46444882989547520,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46443865082298368,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 16325548649218048,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4012 }),
    ]),
      with_prototype: None
    }),
]
} }