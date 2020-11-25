
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 46444217269813248,
        b: 0,
    },
    Scope {
        a: 55450759394557952,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444217269813248,
        b: 0,
    },
    Scope {
        a: 55450759394557952,
        b: 0,
    },
],
  meta_include_prototype: false,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: true,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: true,
      regex: Regex::new("(?=(?<![^\\\\]\\\\)(?<![^\\\\][\\\\]{3})\\2)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }