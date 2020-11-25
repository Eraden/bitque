
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
      regex: Regex::new("(disable|nogc|property|safe|system|trusted)\\b"),
      scope: vec![
        Scope {
            a: 48414439024623616,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 47288521948004534,
            b: 4503599627370496,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1341 }),
        ContextReference::Direct(ContextId { index: 1357 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([\\p{L}_][\\p{L}0-9_]*)\\s*(\\()"),
      scope: vec![
        Scope {
            a: 46444882986795008,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445252353982464,
            b: 0,
        },
        Scope {
            a: 48414576463577088,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288521948004534,
            b: 4503599627370496,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1286 }),
        ContextReference::Direct(ContextId { index: 1341 }),
        ContextReference::Direct(ContextId { index: 1357 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([\\p{L}_][\\p{L}0-9_]*)\\b"),
      scope: vec![
        Scope {
            a: 46445252353982464,
            b: 0,
        },
        Scope {
            a: 48414576463577088,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }