
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
      regex: Regex::new("\\b(0x[0-9A-Fa-f](?>_?[0-9A-Fa-f])*|0b[01]+|0o[0-7]+|\\d(?>_?\\d)*(\\.\\d(?>_?\\d)*)?([eE][-+]?\\d(?>_?\\d)*)?)\\b"),
      scope: vec![
        Scope {
            a: 59955089168859136,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }