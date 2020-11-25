
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 281530824065118,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281530824065118,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: true,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: true,
      regex: Regex::new("\\s*\\2(?m:$)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956395,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6668 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 777 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6669 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6667 })),
]
} }