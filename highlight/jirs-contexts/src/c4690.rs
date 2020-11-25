
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
        index: 4695,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\\\)(?:deqn))(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255171801153,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629362032705,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 51228724934213632,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4679 }),
    ]),
      with_prototype: None
    }),
]
} }