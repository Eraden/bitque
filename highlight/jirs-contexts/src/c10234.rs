
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 10249 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[ \\t]*(\\S(?:.*\\S)?)[ \\t]*(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445080565186719,
            b: 6755399441055744,
        },
        Scope {
            a: 55451949106921496,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }