
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
      regex: Regex::new("(?=\\\'\\\')"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8480 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\\")"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8482 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(~?[a-zA-Z0-9\\.\\_\\-\\+]*(\\/[a-zA-Z0-9\\.\\_\\-\\+]+)+)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 55451949125206144,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8483 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\<[a-zA-Z0-9\\.\\_\\-\\+]+(\\/[a-zA-Z0-9\\.\\_\\-\\+]+)*\\>)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 55451949255753856,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8484 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([a-zA-Z][a-zA-Z0-9\\+\\-\\.]*\\:[a-zA-Z0-9\\%\\/\\?\\:\\@\\&\\=\\+\\$\\,\\-\\_\\.\\!\\~\\*\\\']+)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 55451949132218496,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8485 }),
    ]),
      with_prototype: None
    }),
]
} }