
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
      regex: Regex::new("(#)(?!#)\\s*(?=\\S)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629353709750,
            b: 13792273858822144,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3200 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(##)(?!#)\\s*(?=\\S)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629353709750,
            b: 13792273858822144,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3201 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(#{3,6})(?!#)\\s*(?=\\S)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629353709750,
            b: 13792273858822144,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3202 }),
    ]),
      with_prototype: None
    }),
]
} }