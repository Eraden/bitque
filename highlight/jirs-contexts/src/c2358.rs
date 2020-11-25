
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
        index: 2376,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bexports\\b"),
      scope: vec![
        Scope {
            a: 52636787062539012,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2324 }),
        ContextReference::Direct(ContextId { index: 2323 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bopens\\b"),
      scope: vec![
        Scope {
            a: 52636787062539040,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2370 }),
        ContextReference::Direct(ContextId { index: 2323 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\brequires\\b"),
      scope: vec![
        Scope {
            a: 52636787062539041,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2382 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\buses\\b"),
      scope: vec![
        Scope {
            a: 52636787062539042,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2394 }),
        ContextReference::Direct(ContextId { index: 2367 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bprovides\\b"),
      scope: vec![
        Scope {
            a: 52636787062539043,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2377 }),
        ContextReference::Direct(ContextId { index: 2378 }),
        ContextReference::Direct(ContextId { index: 2367 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2381 })),
]
} }