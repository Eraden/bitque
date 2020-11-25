
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
        index: 3111,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("do(?!(?:[A-Za-z0-9_]))"),
      scope: vec![
        Scope {
            a: 52636636691759104,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3082 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("if(?!(?:[A-Za-z0-9_]))"),
      scope: vec![
        Scope {
            a: 52636636711616559,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3103 }),
        ContextReference::Direct(ContextId { index: 3088 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("while(?!(?:[A-Za-z0-9_]))"),
      scope: vec![
        Scope {
            a: 52636636708536367,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3058 }),
        ContextReference::Direct(ContextId { index: 3088 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("repeat(?!(?:[A-Za-z0-9_]))"),
      scope: vec![
        Scope {
            a: 52636636708536367,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3059 }),
        ContextReference::Direct(ContextId { index: 3060 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("for(?!(?:[A-Za-z0-9_]))"),
      scope: vec![
        Scope {
            a: 52636636708536367,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3061 }),
        ContextReference::Direct(ContextId { index: 3062 }),
        ContextReference::Direct(ContextId { index: 3091 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("::"),
      scope: vec![
        Scope {
            a: 47288629334180022,
            b: 13229323905400832,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3063 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("goto(?!(?:[A-Za-z0-9_]))"),
      scope: vec![
        Scope {
            a: 52636636716335151,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3065 }),
    ]),
      with_prototype: None
    }),
]
} }