
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
      regex: Regex::new("/"),
      scope: vec![
        Scope {
            a: 46444118493167636,
            b: 0,
        },
        Scope {
            a: 47288620732514324,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1496 }),
        ContextReference::Direct(ContextId { index: 1497 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1532 })),
]
} }