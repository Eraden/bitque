
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
      regex: Regex::new("\\b([A-Za-z][A-Za-z0-9_]*)\\b(\\.)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259727250784256,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620745621636,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b([A-Za-z][A-Za-z0-9_]*)\\b"),
      scope: vec![
        Scope {
            a: 59392130643525764,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8648 })),
]
} }