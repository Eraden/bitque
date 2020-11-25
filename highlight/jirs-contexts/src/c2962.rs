
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
        index: 2981,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\\\)includegraphics)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255142768677,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629362032677,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2774 }),
        ContextReference::Direct(ContextId { index: 2948 }),
        ContextReference::Direct(ContextId { index: 2974 }),
    ]),
      with_prototype: None
    }),
]
} }