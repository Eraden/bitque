
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
      regex: Regex::new("(?=(?:[^ \"\'>/=\\x00-\\x1f\\x7f-\\x9f]))"),
      scope: vec![
        Scope {
            a: 59392186477182981,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10180 }),
        ContextReference::Direct(ContextId { index: 10179 }),
        ContextReference::Direct(ContextId { index: 10144 }),
    ]),
      with_prototype: None
    }),
]
} }