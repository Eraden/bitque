
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
  clear_scopes: Some(
    ClearAmount::All,
),
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<\\?(?i:php)?"),
      scope: vec![
        Scope {
            a: 46444208690495546,
            b: 0,
        },
        Scope {
            a: 47288521949642934,
            b: 16325548649218048,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2107 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4002 }),
    )
    }),
]
} }