
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
        index: 5074,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(?:[\\p{L}][_\\p{L}\\p{N}]*|_[_\\p{L}\\p{N}]+)\\b)(?=<)"),
      scope: vec![
        Scope {
            a: 59392130645426248,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5010 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(?:[\\p{L}][_\\p{L}\\p{N}]*|_[_\\p{L}\\p{N}]+)\\b)"),
      scope: vec![
        Scope {
            a: 59392130645426248,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5082 }),
    ]),
      with_prototype: None
    }),
]
} }