
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
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(?:(asserts)\\s+)?(?!asserts)(?:(this)|([_$\\p{L}][_$\\p{L}\\p{N}]*))\\s(is)(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628112181758,
            b: 42221246506598400,
        },
    ]),(2, vec![
        Scope {
            a: 49258876848439296,
            b: 0,
        },
        Scope {
            a: 49259061576401046,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49258876848439296,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636628121289440,
            b: 42221246506598400,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(asserts)\\s+(?!is)(?:(this)|([_$\\p{L}][_$\\p{L}\\p{N}]*))(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628112181758,
            b: 42221246506598400,
        },
    ]),(2, vec![
        Scope {
            a: 49258876848439296,
            b: 0,
        },
        Scope {
            a: 49259061576401046,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49258876848439296,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))asserts(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.))"),
      scope: vec![
        Scope {
            a: 52636628112181758,
            b: 42221246506598400,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))is(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.))"),
      scope: vec![
        Scope {
            a: 52636628121289440,
            b: 42221246506598400,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }