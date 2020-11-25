
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 3928 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3927 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3976 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3914 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(";"),
      scope: vec![
        Scope {
            a: 47288689444323328,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }