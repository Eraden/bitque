
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
        index: 6125,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*((%)ifn?(?:idni?|id|num|str|token|empty|env)?)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466379489367,
            b: 0,
        },
        Scope {
            a: 52636636705390677,
            b: 24488322973827072,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301119,
            b: 23925746682560512,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6108 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*((%)ifn?(?:def|ctx))\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466379489367,
            b: 0,
        },
        Scope {
            a: 52636636705390677,
            b: 24488322973827072,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301119,
            b: 23925746682560512,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6108 }),
        ContextReference::Direct(ContextId { index: 6107 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*((%)rep)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466379489367,
            b: 0,
        },
        Scope {
            a: 52636636705390677,
            b: 24488322973827072,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301119,
            b: 23925746682560512,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6108 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*((%)ifn?macro)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466379489367,
            b: 0,
        },
        Scope {
            a: 52636636705390677,
            b: 24488322973827072,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301119,
            b: 23925746682560512,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6108 }),
        ContextReference::Direct(ContextId { index: 6112 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6111 })),
]
} }