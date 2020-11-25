
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
        index: 5277,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:false|null|true)\\b"),
      scope: vec![
        Scope {
            a: 59955110641991680,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  # 1.1, 1.1e1, 1.1e-1, 1.1d, 1.1e1d, 1.1e-1d, 1.e1d | 1e1 1e1d | 1d\n  \\b\\d+ (?: (?: (\\.) \\d+ (?:[eE][-+]?\\d+)? | (?:[eE][-+]?\\d+) ) ([dDfF])? | ([dDfF]) )\n  # .1, .1e1, .1e-1\n  | (\\.) \\d+ (?:[eE][-+]?\\d+)? ([dDfF])?\n)\\b"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 20547673299877888,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397961,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553289,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576476553289,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288620735397961,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 48414576476553289,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[xX])\\h+([lL]?)\\b"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 20547673299877888,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 20547673299877888,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553289,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:0|[1-9][0-9]*)([lL]?)\\b"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 20547673299877888,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576476553289,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\(\\)"),
      scope: vec![
        Scope {
            a: 59955110641991680,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }