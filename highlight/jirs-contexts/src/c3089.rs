
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
        index: 3111,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=;)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3109 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3083 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3117 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3119 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3107 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3115 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3108 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3118 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3095 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3086 })),
]
} }