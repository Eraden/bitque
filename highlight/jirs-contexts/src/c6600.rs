
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
      regex: Regex::new("([a-zA-Z\\$_]\\w*(\\.\\w+)*)"),
      scope: vec![
        Scope {
            a: 49258932679213056,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49258932679213056,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }