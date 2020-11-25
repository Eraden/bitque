
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
      regex: Regex::new("^(?:\\s*+)(?i)(function|filter|configuration|workflow)\\s+(?:(global|local|script|private):)?((?:\\p{L}|\\d|_|-|\\.)+)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46444131375054848,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 48414576471113728,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414439095795843,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130630615171,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8572 }),
    ]),
      with_prototype: None
    }),
]
} }