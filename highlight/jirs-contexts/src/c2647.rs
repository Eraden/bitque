
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
        a: 844609613725696,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844609613725696,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: Some(
    ContextId {
        index: 2664,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 2546 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\)|\\}|\\]"),
      scope: vec![
        Scope {
            a: 50103314683854891,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2669 })),
]
} }