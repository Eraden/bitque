
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 7369 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b C \\b)"),
      scope: vec![
        Scope {
            a: 61925409852557509,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\,"),
      scope: vec![
        Scope {
            a: 47288620868239360,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b (name) \\s* (=) \\s* ([A-Za-z_][A-Za-z_0-9]*) )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49258876985737216,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628246988997,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130767325381,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }