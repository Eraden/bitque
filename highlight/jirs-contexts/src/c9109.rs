
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
      regex: Regex::new("^---(?m:$)"),
      scope: vec![
        Scope {
            a: 699194515143000064,
            b: 0,
        },
        Scope {
            a: 47288522099654838,
            b: 699465317125980160,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9011 }),
        ContextReference::Direct(ContextId { index: 5881 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 9012 }),
    )
    }),
]
} }