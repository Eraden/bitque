
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
        index: 8656,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("}"),
      scope: vec![
        Scope {
            a: 47288629325660732,
            b: 48132787953205248,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8651 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8643 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8633 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8649 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8631 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8657 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8635 })),
]
} }