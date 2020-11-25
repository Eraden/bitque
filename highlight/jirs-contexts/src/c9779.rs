
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
      regex: Regex::new("(:)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47291305085566976,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9968 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9858 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9936 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([_$\\p{L}][_$\\p{L}\\p{N}]*)"),
      scope: vec![
        Scope {
            a: 49261685762490519,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }