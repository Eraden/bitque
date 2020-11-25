
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
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(new)\\b(?=\\s*\\<)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444251649409174,
            b: 0,
        },
        Scope {
            a: 52636636703686806,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9529 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(new)\\b\\s*(?=\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636703686806,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9530 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(\n  (?=\n    [(]\\s*(\n      ([)]) |\n      (\\.\\.\\.) |\n      ([_$\\p{L}\\p{N}]+\\s*(\n        ([:,?=])|\n        ([)]\\s*=>)\n      ))\n    )\n  )\n)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9531 }),
    ]),
      with_prototype: None
    }),
]
} }