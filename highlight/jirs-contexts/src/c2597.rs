
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
        index: 2664,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\.\\?)?(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788227653632,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288521944400054,
            b: 12103423998558208,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2453 }),
    ]),
      with_prototype: None
    }),
]
} }