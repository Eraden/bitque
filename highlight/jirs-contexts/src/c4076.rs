
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
        a: 59392186470432826,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 59392186470432826,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 4128 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\\\)?(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)(?!\\\\)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620745621562,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }