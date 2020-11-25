
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9873 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(?:(\\bdefault)|(\\*)|(\\b[_$\\p{L}][_$\\p{L}\\p{N}]*))\\s+(as)\\s+(?:(\\bdefault(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.)))|(\\b[_$\\p{L}][_$\\p{L}\\p{N}]*))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636740124823,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955110804324503,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49259087310291095,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636636736848023,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 52636636740124823,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 49259087310291393,
            b: 42502721483309056,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9958 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*"),
      scope: vec![
        Scope {
            a: 59955110804324503,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(default)\\b"),
      scope: vec![
        Scope {
            a: 52636636740124823,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([_$\\p{L}][_$\\p{L}\\p{N}]*)"),
      scope: vec![
        Scope {
            a: 49259087310291393,
            b: 42502721483309056,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }