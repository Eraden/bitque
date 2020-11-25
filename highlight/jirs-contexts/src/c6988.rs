
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
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6966 }),
        ContextReference::Direct(ContextId { index: 7007 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 6967 }),
    )
    }),
]
} }