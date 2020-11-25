
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
      regex: Regex::new("(?=,|>>)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6859 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[a-z][\\w-]*(?:\\((?:(\\d+)|([a-z_]\\w+))\\))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089168859136,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259040053854208,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }