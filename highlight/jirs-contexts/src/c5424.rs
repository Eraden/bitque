
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
  meta_include_prototype: false,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: true,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: true,
      regex: Regex::new("^\\s*\\4(\\s+)\\n"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314747686986,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: true,
      regex: Regex::new("^\\s*(\\4)(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636782788682,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }