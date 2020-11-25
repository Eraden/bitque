
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
        index: 8688,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\n  (?:\\+|-)?(?:0|[1-9]\\d*)?\\.\\d+(?i:e(?:\\+|-)?(?:\\+|-)?(?:0|[1-9]\\d*))?(f?)  # literal with a dot: .3 .3e+4 1.3 1.3f\n  |(?:\\+|-)?(?:0|[1-9]\\d*)(f)                # literal with a \'f\': 1f\n  |(?:\\+|-)?(?:0|[1-9]\\d*)(?i:e(?:\\+|-)?(?:\\+|-)?(?:0|[1-9]\\d*))(f?)        # literal with an exp: .3e+4 1e-3 1e3f\n)\\b"),
      scope: vec![
        Scope {
            a: 59955089176592517,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070553,
            b: 37436171902517248,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325070553,
            b: 37436171902517248,
        },
    ]),(3, vec![
        Scope {
            a: 47288629325070553,
            b: 37436171902517248,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\+|-)?(?:0|[1-9]\\d*)\\b"),
      scope: vec![
        Scope {
            a: 59955089176461445,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(0)[0-8]+\\b"),
      scope: vec![
        Scope {
            a: 59955089185570949,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070690,
            b: 37436171902517248,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(0[xX])[0-9A-Fa-f]+\\b"),
      scope: vec![
        Scope {
            a: 59955089201168517,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070928,
            b: 37436171902517248,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }