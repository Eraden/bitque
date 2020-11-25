
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
        index: 5435,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*\\b(alias)(?![-=\\w])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255114981450,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5456 }),
        ContextReference::Direct(ContextId { index: 5458 }),
        ContextReference::Direct(ContextId { index: 5461 }),
        ContextReference::Direct(ContextId { index: 5454 }),
        ContextReference::Direct(ContextId { index: 5452 }),
        ContextReference::Direct(ContextId { index: 5453 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*\\b(typeset|declare|local)(?![-=\\w])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439028424704,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5456 }),
        ContextReference::Direct(ContextId { index: 5458 }),
        ContextReference::Direct(ContextId { index: 5461 }),
        ContextReference::Direct(ContextId { index: 5454 }),
        ContextReference::Direct(ContextId { index: 5459 }),
        ContextReference::Direct(ContextId { index: 5455 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*\\b(export)(?![-=\\w])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439028424704,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5456 }),
        ContextReference::Direct(ContextId { index: 5458 }),
        ContextReference::Direct(ContextId { index: 5461 }),
        ContextReference::Direct(ContextId { index: 5454 }),
        ContextReference::Direct(ContextId { index: 5459 }),
        ContextReference::Direct(ContextId { index: 5457 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*\\b(readonly)(?![-=\\w])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439028424704,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5456 }),
        ContextReference::Direct(ContextId { index: 5458 }),
        ContextReference::Direct(ContextId { index: 5461 }),
        ContextReference::Direct(ContextId { index: 5454 }),
        ContextReference::Direct(ContextId { index: 5459 }),
        ContextReference::Direct(ContextId { index: 5460 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\s*[^{}()=\\s]*(?:[({][^{}()=\\s]*[)}])?[^{}()=\\s]*=)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5461 }),
        ContextReference::Direct(ContextId { index: 5454 }),
        ContextReference::Direct(ContextId { index: 5459 }),
    ]),
      with_prototype: None
    }),
]
} }