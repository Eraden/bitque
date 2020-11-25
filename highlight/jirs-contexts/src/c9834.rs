
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9931 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9908 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9909 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9893 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9979 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\.\\.\\."),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 52636628152877207,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9835 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9958 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9959 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9977 })),
]
} }