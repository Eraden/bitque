
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
        index: 7635,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\)"),
      scope: vec![
        Scope {
            a: 47288521944400043,
            b: 32088147345014784,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$)((?:(?:[_A-Za-z])(?:[_0-9A-Za-z])*(?!(?:[_0-9A-Za-z]))))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514546,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49258876846080000,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7609 }),
        ContextReference::Direct(ContextId { index: 7610 }),
    ]),
      with_prototype: None
    }),
]
} }