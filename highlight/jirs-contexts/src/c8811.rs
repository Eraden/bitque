
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
      regex: Regex::new("(?m:$)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([\\w\\d\\.]+)\\s+(\\d+\\.\\d+)(?:\\s+(as)\\s+([A-Z][\\w\\d]*))?"),
      scope: vec![
        Scope {
            a: 46445256672411784,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632319112,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955089171283968,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636787041304712,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130632319112,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\\"[^\\\"]+\\\")(?:\\s+(as)\\s+([A-Z][\\w\\d]*))?"),
      scope: vec![
        Scope {
            a: 46445256810168456,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 55451420828565640,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787041304712,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632319112,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }