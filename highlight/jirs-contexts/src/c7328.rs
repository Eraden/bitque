
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
      regex: Regex::new("(?xi)\n(?:^\n  \\s* (else\\s*if)     # 1 keyword\n  \\s* \\(              #   opening parenthesis\n)\n"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46453223959363584,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52636636839086277,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }