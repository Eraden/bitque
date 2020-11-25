
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
      regex: Regex::new("\\bpackage\\b(?!::)"),
      scope: vec![
        Scope {
            a: 48414576487039037,
            b: 0,
        },
        Scope {
            a: 52638212966187069,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4328 }),
        ContextReference::Direct(ContextId { index: 4330 }),
        ContextReference::Direct(ContextId { index: 4329 }),
    ]),
      with_prototype: None
    }),
]
} }