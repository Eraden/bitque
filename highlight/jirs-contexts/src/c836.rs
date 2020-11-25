
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 1034 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*((@)(-webkit-|-moz-|-o-)?keyframes)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636720398826,
            b: 3940649673949184,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323300878,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925375376818190,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 837 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521959145486,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 839 }),
    ]),
      with_prototype: None
    }),
]
} }