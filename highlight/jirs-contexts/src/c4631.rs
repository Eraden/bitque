
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
      regex: Regex::new("\\b(yield)(?:\\s+(from))?\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636701197123,
            b: 17451448556060672,
        },
    ]),(2, vec![
        Scope {
            a: 52636636701197600,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }