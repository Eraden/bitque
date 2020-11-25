
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
      regex: Regex::new("((:)(?:nth-child|nth-last-child|nth-of-type|nth-last-of-type|nth-match|nth-last-match|nth-column|nth-last-column))\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392186477183482,
            b: 40813871623045120,
        },
    ]),(2, vec![
        Scope {
            a: 701436475200438417,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 701436475203126157,
            b: 40813871623045120,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9196 }),
    ]),
      with_prototype: None
    }),
]
} }