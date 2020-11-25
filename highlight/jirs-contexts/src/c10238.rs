
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
      regex: Regex::new("^\\s*(#)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711032873119,
            b: 6755399441055744,
        },
        Scope {
            a: 47288629323038879,
            b: 6755399441055744,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10232 }),
    ]),
      with_prototype: None
    }),
]
} }