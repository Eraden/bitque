
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
        index: 5074,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((?:\\d[\\d_]*)?\\d\\.)(\\d[\\d_]*(?:[eE][+-]?[\\d_]*\\d[\\d_]*)?)(f32|f64)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176592456,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955089176592456,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576476553288,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((?:\\d[\\d_]*)?\\d\\.)(?!\\.)"),
      scope: vec![
        Scope {
            a: 59955089176592456,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(\\d[\\d_]*)(f32|f64)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176592456,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553288,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }