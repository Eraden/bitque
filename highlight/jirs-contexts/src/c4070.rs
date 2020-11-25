
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
      regex: Regex::new("(\')(/)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956406,
            b: 16325548649218048,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323957362,
            b: 51228694869442560,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(/)([eimsuxADJSUX]*)(\')"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323957362,
            b: 48132470125625344,
        },
    ]),(2, vec![
        Scope {
            a: 46448263136346170,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323956395,
            b: 16325548649218048,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4071 }),
        ContextReference::Direct(ContextId { index: 4173 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4072 }),
    )
    }),
]
} }