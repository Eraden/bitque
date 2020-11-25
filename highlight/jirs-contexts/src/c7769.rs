
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
      regex: Regex::new("(?![_$a-zA-Z][$\\w.]*)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7854 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7855 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([_$a-zA-Z][$\\w.]*\\.)?([_$a-zA-Z][$\\w]*)"),
      scope: vec![],
      captures: Some(vec![(2, vec![
        Scope {
            a: 49258881157431339,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }