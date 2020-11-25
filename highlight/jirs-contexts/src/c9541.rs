
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
      regex: Regex::new("(>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629477507243,
            b: 42221246506598400,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9578 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(extends)(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.))"),
      scope: vec![
        Scope {
            a: 48414439033405440,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9675 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9653 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(=)(?!>)"),
      scope: vec![
        Scope {
            a: 52636628111130774,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }