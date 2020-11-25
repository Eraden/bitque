
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
        a: 46453266909036544,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46453266909036544,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 7369 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:((?xi:[abcdefghijklmnopqrstuvwxyz_$])) \\s* (-) \\s* ((?xi:[abcdefghijklmnopqrstuvwxyz_$])))"),
      scope: vec![
        Scope {
            a: 46453271204003840,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955200849414341,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628116637893,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955200849414341,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:(?xi:[abcdefghijklmnopqrstuvwxyz_$]))"),
      scope: vec![
        Scope {
            a: 59955200849414341,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }