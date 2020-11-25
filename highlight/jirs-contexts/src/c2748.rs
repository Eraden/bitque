
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
      regex: Regex::new("(>)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956395,
            b: 7881458261688320,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(.*/.*\\.pdf)"),
      scope: vec![
        Scope {
            a: 61925255141720101,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130630615610,
            b: 10414574138294272,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }