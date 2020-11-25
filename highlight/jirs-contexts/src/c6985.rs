
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
        a: 46452828688614044,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46452828688614044,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 7018 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6993 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7017 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6991 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(?=<\\?xml\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6962 }),
        ContextReference::Direct(ContextId { index: 5819 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 6963 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(?=(?:<html|<!DOCTYPE html)\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6964 }),
        ContextReference::Direct(ContextId { index: 2107 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 6965 }),
    )
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6981 })),
]
} }