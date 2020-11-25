
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
      regex: Regex::new("(?xi)(?:\n  \\b(declare \\s+ simd)\\b\n  \\s*\n  \\( \\s* ([A-Za-z_][A-Za-z_0-9]*) \\s* \\)\n)\n"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636297533390848,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615151,
            b: 631911322715422720,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7421 })),
]
} }