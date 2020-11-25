
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
        index: 5884,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(\\|)|(>))([1-9])?([-+])?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636701197787,
            b: 64739592535801856,
        },
    ]),(2, vec![
        Scope {
            a: 52636636701197787,
            b: 422212812958334976,
        },
    ]),(3, vec![
        Scope {
            a: 59955089176461530,
            b: 422494287935045632,
        },
    ]),(4, vec![
        Scope {
            a: 48414439122010193,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5841 }),
    ]),
      with_prototype: None
    }),
]
} }