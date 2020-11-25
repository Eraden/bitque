
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
      regex: Regex::new("\\)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288521992634539,
            b: 12666373951979520,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([a-zA-Z]+)\\s*(\\=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 55451949115834413,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620737429549,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2736 }),
    ]),
      with_prototype: None
    }),
]
} }