
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
        a: 55450759479820382,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 55450759479820382,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((/[eimnosux]*))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 55450759479820382,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956318,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6684 })),
]
} }