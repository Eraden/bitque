
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9873 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9989 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9900 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9965 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9875 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*"),
      scope: vec![
        Scope {
            a: 52638522221068439,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }