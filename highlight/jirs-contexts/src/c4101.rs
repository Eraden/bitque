
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
      regex: Regex::new("(?i)(\\b(?:abstract|final)\\s+)?\\b(class)\\s+(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439098417210,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576475832378,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632319034,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4104 }),
    ]),
      with_prototype: None
    }),
]
} }