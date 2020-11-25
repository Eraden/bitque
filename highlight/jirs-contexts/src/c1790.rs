
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
        a: 281565172662272,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281565172662272,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: Some(
    ContextId {
        index: 1792,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(?=\\[\\s*color\\s+\\\")"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1786 }),
        ContextReference::Direct(ContextId { index: 1793 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(?=\\[)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1787 }),
        ContextReference::Direct(ContextId { index: 1793 }),
    ]),
      with_prototype: None
    }),
]
} }