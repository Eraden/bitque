
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
      regex: Regex::new("(\\\\x\\h{2})|(\\\\[\\\\\"\'abfnrtv])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955200847315536,
            b: 34058472181989376,
        },
    ]),(2, vec![
        Scope {
            a: 59955200847315065,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }