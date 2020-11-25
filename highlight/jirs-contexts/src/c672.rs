
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
      regex: Regex::new("\\b(using)\\b(?=\\s+typename\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636689465344,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(using)\\b\\s+(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)(?!\\s*(<|::))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636689465344,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632450464,
            b: 3377699720527872,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }