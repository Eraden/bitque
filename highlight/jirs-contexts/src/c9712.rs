
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
      regex: Regex::new("\\}"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629325660311,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9873 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9877 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9931 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9908 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9893 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9968 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9979 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10000 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9854 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9956 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9862 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9855 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9875 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9888 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9958 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9959 })),
]
} }