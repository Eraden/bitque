
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9582 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9695 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9604 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9573 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9620 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9591 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9627 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9676 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9612 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9611 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9592 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(declare|export)(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.))"),
      scope: vec![
        Scope {
            a: 48414439033405440,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }