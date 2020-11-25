
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9693 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([&|])(?=\\s*\\{)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 52636628112179350,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9539 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[&|]"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 52636628112179350,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9540 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))keyof(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.))"),
      scope: vec![
        Scope {
            a: 52636628121291259,
            b: 42221246506598400,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\?|\\:)"),
      scope: vec![
        Scope {
            a: 52636628123320470,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))infer(?=\\s+[_$\\p{L}])"),
      scope: vec![
        Scope {
            a: 52636628121291260,
            b: 42221246506598400,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))awaited(?=\\s+[_$\\p{L}])"),
      scope: vec![
        Scope {
            a: 52636628121291261,
            b: 42221246506598400,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))import(?=\\s*\\()"),
      scope: vec![
        Scope {
            a: 52636628121289143,
            b: 42221246506598400,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }