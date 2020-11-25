
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
        a: 46453159534854144,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46453159534854144,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 7369 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7338 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*"),
      scope: vec![
        Scope {
            a: 47290532128229573,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b (\\w+) \\s* (=))"),
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
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7336 })),
]
} }