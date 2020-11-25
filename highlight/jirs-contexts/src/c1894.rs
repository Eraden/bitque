
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
      regex: Regex::new("(?:\\d+(?:_\\d+)*)(?:(\\.)(?:\\d+(?:_\\d+)*)?)?(?:[eE][-+]??(?:\\d+(?:_\\d+)*))?(i)"),
      scope: vec![
        Scope {
            a: 59955089199268058,
            b: 8725724278030336,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397919,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553247,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(0[xX])(?:_?\\h+(?:_\\h+)*)?(?:(\\.)(?:_?\\h+(?:_\\h+)*)?)?(?:[pP][-+]??(?:\\d+(?:_\\d+)*))?(i)"),
      scope: vec![
        Scope {
            a: 59955089199268056,
            b: 8725724278030336,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 8725724278030336,
        },
    ]),(2, vec![
        Scope {
            a: 47288620735397919,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576476553247,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(0[oO])(?:_?[0-7]+(?:_[0-7]+)*)(i)"),
      scope: vec![
        Scope {
            a: 59955089199268194,
            b: 8725724278030336,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 8725724278030336,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553247,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(0[bB])(?:_?[01]+(?:_[01]+)*)(i)"),
      scope: vec![
        Scope {
            a: 59955089199268269,
            b: 8725724278030336,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 8725724278030336,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553247,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }