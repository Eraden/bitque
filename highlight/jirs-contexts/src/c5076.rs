
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
        index: 5074,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(r)((#*)\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475439176,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956406,
            b: 20266198323167232,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4998 }),
    ]),
      with_prototype: None
    }),
]
} }