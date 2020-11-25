
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 5896 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5895 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5900 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5897 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5912 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5902 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5908 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5910 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=[\\S])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }