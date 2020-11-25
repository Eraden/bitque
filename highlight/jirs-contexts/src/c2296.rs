
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
        index: 2376,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\@"),
      scope: vec![
        Scope {
            a: 47288629337129000,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2225 }),
        ContextReference::Direct(ContextId { index: 2293 }),
        ContextReference::Direct(ContextId { index: 2226 }),
        ContextReference::Direct(ContextId { index: 2295 }),
    ]),
      with_prototype: None
    }),
]
} }