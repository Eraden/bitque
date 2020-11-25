
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
      regex: Regex::new("\\b(0[bB])(?!_)([01]|_(?!_))*\\b"),
      scope: vec![
        Scope {
            a: 59955089176461741,
            b: 16325548649218048,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070765,
            b: 16325548649218048,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[xX])(?!_)(\\h|_(?!_))*\\b"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 16325548649218048,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070552,
            b: 16325548649218048,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  (?:\n    (?# such as 123.4 or .123)\n    (?:\\b(?!_)(?:\\d|_(?!_))+|\\B)\\.(?!_)(?:\\d|_(?!_))+\n    |\n    (?# such as 123.)\n    \\b(?!_)(?:\\d|_(?!_))+\\.\n  )(?:[eE][+-]?(?!_)(?:\\d|_(?!_))+)?\\b\n  |\n  (?# such as 123e-4)\n  \\b(?!_)(?:\\d|_(?!_))+(?:[eE][+-]?(?!_)(?:\\d|_(?!_))+)\\b\n)"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 16325548649218048,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?!_)(?:\\d|_(?!_))+\\b"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 16325548649218048,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }