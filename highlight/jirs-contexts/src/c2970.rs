
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 2988 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2985 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2982 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2952 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3007 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3013 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2969 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2954 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3015 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3018 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3014 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3017 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2959 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\})"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }