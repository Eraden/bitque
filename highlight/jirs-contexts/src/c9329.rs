
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
      regex: Regex::new("[ \\t]*(=)[ \\t]*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629327364244,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 9333 }),
        ContextReference::Direct(ContextId { index: 9317 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[ \\t]*(?m:$)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".+(?m:$)"),
      scope: vec![
        Scope {
            a: 50103314818465940,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }