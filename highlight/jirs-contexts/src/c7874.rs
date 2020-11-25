
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 844936031240192,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844936031240192,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("({%)\\s*(raw)\\s*(%})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392186461390349,
            b: 56294995342131200,
        },
    ]),(2, vec![
        Scope {
            a: 52636636696477696,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392186461390349,
            b: 56294995342131200,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7865 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("{#-?"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 59392186461390349,
            b: 51509920738050048,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7866 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("{{-?"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 59392186461390349,
            b: 49258120924364800,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7867 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("{%-?"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 59392186461390349,
            b: 56294995342131200,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7868 }),
    ]),
      with_prototype: None
    }),
]
} }