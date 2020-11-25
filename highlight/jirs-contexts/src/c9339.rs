
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
        a: 46444230200197301,
        b: 41658296553177088,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444230200197301,
        b: 41658296553177088,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[ \\t]*(\\]\\])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629360525493,
            b: 48132856672681984,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9341 })),
]
} }