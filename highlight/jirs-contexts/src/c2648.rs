
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
  meta_include_prototype: false,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2611 }),
        ContextReference::Direct(ContextId { index: 2600 }),
        ContextReference::Direct(ContextId { index: 2607 }),
        ContextReference::Direct(ContextId { index: 2604 }),
        ContextReference::Direct(ContextId { index: 2650 }),
        ContextReference::Direct(ContextId { index: 2649 }),
        ContextReference::Direct(ContextId { index: 2599 }),
    ]),
      with_prototype: None
    }),
]
} }