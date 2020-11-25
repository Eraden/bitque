
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
      regex: Regex::new("\\bvoid\\b"),
      scope: vec![
        Scope {
            a: 48414576511287336,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2354 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(?:[\\p{L}_$][\\p{L}\\p{N}_$]*)\\s*\\()"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2354 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(?:[_$]*\\p{Ll}[\\p{Ll}\\p{N}_$]*\\b)\\s*\\.)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2325 }),
        ContextReference::Direct(ContextId { index: 2286 }),
        ContextReference::Direct(ContextId { index: 2365 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:\\p{Lu}[\\p{L}\\p{N}_$]*)"),
      scope: vec![
        Scope {
            a: 61925366757326848,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2325 }),
        ContextReference::Direct(ContextId { index: 2286 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:boolean|byte|char|short|int|float|long|double)\\b"),
      scope: vec![
        Scope {
            a: 48414576511352872,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2325 }),
        ContextReference::Direct(ContextId { index: 2299 }),
    ]),
      with_prototype: None
    }),
]
} }