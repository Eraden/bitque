
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
        index: 1314,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(0[bB])(_?)(?:[01][01_]*)(L[uU]|[uU]L|[LuU])?"),
      scope: vec![
        Scope {
            a: 59955089176461741,
            b: 4503599627370496,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 4503599627370496,
        },
    ]),(2, vec![
        Scope {
            a: 50103314667667472,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576476553232,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(0[xX])(_?)(?:\\h[\\h_]*)(L[uU]|[uU]L|[LuU])?"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 4503599627370496,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 4503599627370496,
        },
    ]),(2, vec![
        Scope {
            a: 50103314667667472,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576476553232,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:0_*|[1-9][0-9_]*)(L[uU]|[uU]L|[LuU])?"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 4503599627370496,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576476553232,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }