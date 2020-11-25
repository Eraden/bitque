
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
      regex: Regex::new("((\\\\)[hvmf]box)\\s*(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255142572069,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629362032677,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2766 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\\\)(?:framebox|makebox))\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255142572069,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629362032677,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2767 }),
        ContextReference::Direct(ContextId { index: 2948 }),
        ContextReference::Direct(ContextId { index: 2974 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\\\)parbox)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255142572069,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629362032677,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2768 }),
        ContextReference::Direct(ContextId { index: 2948 }),
        ContextReference::Direct(ContextId { index: 2948 }),
        ContextReference::Direct(ContextId { index: 2974 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\\\)raisebox)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255142572069,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629362032677,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2769 }),
        ContextReference::Direct(ContextId { index: 2948 }),
        ContextReference::Direct(ContextId { index: 2974 }),
    ]),
      with_prototype: None
    }),
]
} }