
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
      regex: Regex::new("(?:(\\?)\\s*)?(:)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628252886041,
            b: 12103423998558208,
        },
    ]),(2, vec![
        Scope {
            a: 52636628252885390,
            b: 12103423998558208,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7741 }),
    ]),
      with_prototype: None
    }),
]
} }