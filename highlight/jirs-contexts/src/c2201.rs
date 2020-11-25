
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
        index: 2203,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("-?(?:0|[1-9]\\d*)(?:(?:(\\.)\\d+)(?:[eE][-+]?\\d+)?|(?:[eE][-+]?\\d+))"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 10696049115004928,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397926,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("-?(?:0|[1-9]\\d*)"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 10696049115004928,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }