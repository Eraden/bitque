
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9101 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9100 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9099 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9142 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9098 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9128 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9149 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9140 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9139 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9146 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9141 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(false|true|null)\\b"),
      scope: vec![
        Scope {
            a: 59955110646448128,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9123 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9156 })),
]
} }