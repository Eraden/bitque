
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
        index: 7635,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("schema(?!(?:[_0-9A-Za-z]))"),
      scope: vec![
        Scope {
            a: 52638213095030898,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7638 }),
        ContextReference::Direct(ContextId { index: 7621 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("scalar(?!(?:[_0-9A-Za-z]))"),
      scope: vec![
        Scope {
            a: 52638213032575090,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7621 }),
        ContextReference::Direct(ContextId { index: 7637 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("type(?!(?:[_0-9A-Za-z]))"),
      scope: vec![
        Scope {
            a: 52638212955111538,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7644 }),
        ContextReference::Direct(ContextId { index: 7621 }),
        ContextReference::Direct(ContextId { index: 7645 }),
        ContextReference::Direct(ContextId { index: 7646 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("interface(?!(?:[_0-9A-Za-z]))"),
      scope: vec![
        Scope {
            a: 52638212955111837,
            b: 32088147345014784,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7644 }),
        ContextReference::Direct(ContextId { index: 7621 }),
        ContextReference::Direct(ContextId { index: 7632 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("union(?!(?:[_0-9A-Za-z]))"),
      scope: vec![
        Scope {
            a: 52638212955111837,
            b: 32088147345014784,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7594 }),
        ContextReference::Direct(ContextId { index: 7621 }),
        ContextReference::Direct(ContextId { index: 7650 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("enum(?!(?:[_0-9A-Za-z]))"),
      scope: vec![
        Scope {
            a: 52638212955111787,
            b: 32088147345014784,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7623 }),
        ContextReference::Direct(ContextId { index: 7621 }),
        ContextReference::Direct(ContextId { index: 7624 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("input(?!(?:[_0-9A-Za-z]))"),
      scope: vec![
        Scope {
            a: 52638212955112337,
            b: 32088147345014784,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7629 }),
        ContextReference::Direct(ContextId { index: 7621 }),
        ContextReference::Direct(ContextId { index: 7630 }),
    ]),
      with_prototype: None
    }),
]
} }