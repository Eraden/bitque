
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
        a: 46444883013993111,
        b: 5910974510923776,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444883013993111,
        b: 5910974510923776,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\)"),
      scope: vec![
        Scope {
            a: 46444883013993111,
            b: 5910974510923776,
        },
        Scope {
            a: 47288521948004523,
            b: 186617999753478144,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1780 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("reset"),
      scope: vec![
        Scope {
            a: 61925409747894293,
            b: 7036874417766400,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }