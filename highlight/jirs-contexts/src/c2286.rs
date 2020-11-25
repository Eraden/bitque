
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
      regex: Regex::new("(?=<)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2299 }),
        ContextReference::Direct(ContextId { index: 2331 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\.(?!\\.)"),
      scope: vec![
        Scope {
            a: 47288788226932776,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2363 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2299 })),
]
} }