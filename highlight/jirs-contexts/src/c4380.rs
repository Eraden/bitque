
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
      regex: Regex::new("(\\b[_\\p{L}]\\w*\\b)?(::)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259727246131200,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788251050045,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4265 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4378 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4376 })),
]
} }