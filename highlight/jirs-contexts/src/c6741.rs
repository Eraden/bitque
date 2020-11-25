
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
        a: 844841541959680,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844841541959680,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 6735 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(?i:arg)\\s"),
      scope: vec![
        Scope {
            a: 52636636695035904,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6739 })),
]
} }