
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
      regex: Regex::new("\\b(\\w+)\\s+(=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49258876902375601,
            b: 14355223812243456,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130675,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\["),
      scope: vec![
        Scope {
            a: 47288521961570355,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3380 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(=)?\\s*\\b(\\w+)\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628111130675,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615091,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288521948004534,
            b: 14355223812243456,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3381 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(\\w+)\\s*(?=%|(?m:$))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130630615091,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }