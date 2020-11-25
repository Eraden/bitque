
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
      regex: Regex::new("\\b(async|seq|promise|task|maybe|asyncMaybe|controller|scope|application|pipeline)\\s*\\{"),
      scope: vec![
        Scope {
            a: 622060149502115840,
            b: 0,
        },
    ],
      captures: Some(vec![(0, vec![
        Scope {
            a: 52636271616458752,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }