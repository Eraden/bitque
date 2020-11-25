
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
  prototype: Some(
    ContextId {
        index: 1627,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(-)\\s*(include(_lib)?)(?=[^[_A-Za-z\\d@]])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323300884,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636636728263110,
            b: 5629499534213120,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1459 }),
        ContextReference::Direct(ContextId { index: 1460 }),
    ]),
      with_prototype: None
    }),
]
} }