
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
      regex: Regex::new("(\\3)(?:(\")|(.))([cwd]?)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955136443383824,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956395,
            b: 4503599627370496,
        },
    ]),(3, vec![
        Scope {
            a: 50103314654691328,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 48414576475439120,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }