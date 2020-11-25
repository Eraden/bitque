
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
        a: 844772822482944,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844772822482944,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: Some(
    ContextId {
        index: 5884,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 5865 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^---"),
      scope: vec![
        Scope {
            a: 59392186485571766,
            b: 22799473113563136,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\.{3}"),
      scope: vec![
        Scope {
            a: 59392186485571755,
            b: 22799473113563136,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5882 })),
]
} }