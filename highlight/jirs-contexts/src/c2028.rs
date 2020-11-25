
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
      regex: Regex::new("(\\w+)(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444084123992064,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629358624950,
            b: 9288674231451648,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1993 }),
    ]),
      with_prototype: None
    }),
]
} }