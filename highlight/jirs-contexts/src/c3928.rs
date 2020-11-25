
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
      regex: Regex::new("\\b(__attribute__)\\s*(\\(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439027310592,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46443865082232832,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 16044073672507392,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3839 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(__declspec)(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439027310592,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46443865082232832,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 16044073672507392,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3840 }),
    ]),
      with_prototype: None
    }),
]
} }