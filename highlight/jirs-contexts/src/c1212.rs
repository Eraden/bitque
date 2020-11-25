
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
        index: 1314,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(this)\\s*(\\()\\s*(this)\\s*(\\))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444131367518208,
            b: 0,
        },
        Scope {
            a: 59392130630615602,
            b: 4503599627370496,
        },
    ]),(2, vec![
        Scope {
            a: 46444131382984720,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 4503599627370496,
        },
    ]),(3, vec![
        Scope {
            a: 46444131382984720,
            b: 0,
        },
        Scope {
            a: 49259061523251200,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 46444131382984720,
            b: 0,
        },
        Scope {
            a: 47288521944400043,
            b: 4503599627370496,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1251 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(this)\\s*(?=\\(|(?m:$))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444131367518208,
            b: 0,
        },
        Scope {
            a: 59392130630615404,
            b: 4503599627370496,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1214 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("~\\s*this"),
      scope: vec![
        Scope {
            a: 46444131367518208,
            b: 0,
        },
        Scope {
            a: 59392130630615473,
            b: 4503599627370496,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1218 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1303 })),
]
} }