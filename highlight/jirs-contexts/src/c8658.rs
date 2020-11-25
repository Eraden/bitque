
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
        index: 8656,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(rpc)\\b\\s*\\b([A-Za-z][A-Za-z0-9_]*)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576622764164,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630617485,
            b: 37154696925806592,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8659 }),
        ContextReference::Direct(ContextId { index: 8660 }),
        ContextReference::Direct(ContextId { index: 8661 }),
        ContextReference::Direct(ContextId { index: 8660 }),
    ]),
      with_prototype: None
    }),
]
} }