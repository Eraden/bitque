
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
        index: 2422,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("({)((@)(?:code|literal))(?:\\s+|(?=\\}))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521989750966,
            b: 11540474045136896,
        },
    ]),(2, vec![
        Scope {
            a: 52636787039011628,
            b: 11540474045136896,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323300905,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2402 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("({)((@)link(?:plain)?)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521989750966,
            b: 11540474045136896,
        },
    ]),(2, vec![
        Scope {
            a: 52636787039010843,
            b: 11540474045136896,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323300905,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2416 }),
        ContextReference::Direct(ContextId { index: 2419 }),
        ContextReference::Direct(ContextId { index: 2424 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("({)((@)value)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521989750966,
            b: 11540474045136896,
        },
    ]),(2, vec![
        Scope {
            a: 52636787039011183,
            b: 11540474045136896,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323300905,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2416 }),
        ContextReference::Direct(ContextId { index: 2424 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("({)((@)(?:index|extLink))\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521989750966,
            b: 11540474045136896,
        },
    ]),(2, vec![
        Scope {
            a: 52636787039010857,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323300905,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2416 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("({)((@)inheritDoc)(})"),
      scope: vec![
        Scope {
            a: 46446837198553088,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521989750966,
            b: 11540474045136896,
        },
    ]),(2, vec![
        Scope {
            a: 52636787039010857,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323300905,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288521989750955,
            b: 11540474045136896,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }