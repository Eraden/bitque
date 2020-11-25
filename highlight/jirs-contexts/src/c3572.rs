
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
      regex: Regex::new("\\]"),
      scope: vec![
        Scope {
            a: 47288522009084075,
            b: 15762598695796736,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(NSPredicate)\\s+(predicateWithFormat(:))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925366826991672,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 61925255157907512,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288620749357112,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3573 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s+(\\w+)(?=\\s*\\])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255157907512,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s+(\\w+(:))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255157907512,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620749357112,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3574 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3731 })),
]
} }