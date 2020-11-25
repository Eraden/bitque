
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
      regex: Regex::new("[?*+]\\??"),
      scope: vec![
        Scope {
            a: 52636628154253356,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{(\\d+)(?:(,)(\\d+)?)?\\}"),
      scope: vec![
        Scope {
            a: 52636628154253356,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955157895872556,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620757155884,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955157895872556,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{(,)(\\d+)\\}"),
      scope: vec![
        Scope {
            a: 52636628154253356,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620757155884,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955157895872556,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }