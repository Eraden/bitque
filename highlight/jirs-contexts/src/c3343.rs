
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
      regex: Regex::new("([ ]*)(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314663866417,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629373173942,
            b: 13792273858822144,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3268 }),
    ]),
      with_prototype: None
    }),
]
} }