
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 46444131392553311,
        b: 17451448556060672,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444131392553311,
        b: 17451448556060672,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("->"),
      scope: vec![
        Scope {
            a: 47288620747194719,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4586 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=:)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4578 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4608 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4571 })),
]
} }