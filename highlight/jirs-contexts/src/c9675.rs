
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9578 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9665 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9630 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9691 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9680 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9688 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9692 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9686 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9681 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9687 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9682 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9689 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9683 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(readonly)(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.))\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439033405440,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9685 })),
]
} }