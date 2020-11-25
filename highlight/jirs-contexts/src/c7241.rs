
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
      regex: Regex::new("(&&)|(\\|\\|)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46444882998067390,
            b: 29836347531329536,
        },
    ]),(1, vec![
        Scope {
            a: 52636628111198396,
            b: 29836347531329536,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111198397,
            b: 29836347531329536,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 7157 }),
    ]),
      with_prototype: None
    }),
]
} }