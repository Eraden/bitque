
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
        index: 2981,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\\\)documentclass\\b"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 52636636746940453,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629362032677,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2927 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\\\)usepackage\\b"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 52636636746940453,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629362032677,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2929 }),
    ]),
      with_prototype: None
    }),
]
} }