
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
  prototype: Some(
    ContextId {
        index: 5074,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\)"),
      scope: vec![
        Scope {
            a: 46444882990465024,
            b: 0,
        },
        Scope {
            a: 46443865083215872,
            b: 0,
        },
        Scope {
            a: 47288521944400043,
            b: 20266198323167232,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:(?:[\\p{L}][_\\p{L}\\p{N}]*|_[_\\p{L}\\p{N}]+)\\b))(\\()"),
      scope: vec![
        Scope {
            a: 46444882990465024,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49258881138294784,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46443865083215872,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 20266198323167232,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4962 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(","),
      scope: vec![
        Scope {
            a: 47288620725829632,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("="),
      scope: vec![
        Scope {
            a: 52636628103462912,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5081 })),
]
} }