
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
      regex: Regex::new("(?={)"),
      scope: vec![],
      captures: Some(vec![]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8708 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8721 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8711 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(inherits)\\b\\s+"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439032356864,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8694 }),
    ]),
      with_prototype: None
    }),
]
} }