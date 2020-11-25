
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
      regex: Regex::new("(\\[)\\s*(\\])"),
      scope: vec![
        Scope {
            a: 46445668982719197,
            b: 17451448556060672,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521971925174,
            b: 17451448556060672,
        },
    ]),(2, vec![
        Scope {
            a: 47288521971925163,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4544 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\["),
      scope: vec![
        Scope {
            a: 47288521971925174,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4597 }),
        ContextReference::Direct(ContextId { index: 4545 }),
    ]),
      with_prototype: None
    }),
]
} }