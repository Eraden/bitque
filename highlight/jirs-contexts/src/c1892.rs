
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
      regex: Regex::new("(0[xX])(?:_?\\h+(?:_\\h+)*)"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 8725724278030336,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 8725724278030336,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }