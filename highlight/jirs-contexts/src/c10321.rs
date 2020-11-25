
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
      regex: Regex::new("^\\s*([a-zA-Z1]+)\\b[ \\t]*(=)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445780653244579,
            b: 0,
        },
        Scope {
            a: 52636787023216640,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130787,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10311 }),
    ]),
      with_prototype: None
    }),
]
} }