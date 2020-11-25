
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
        a: 46444328948334696,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444328948334696,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^--((?:[\\w=?:-]*[a-zA-Z0-9][\\w=?:-]*))--(?:\\n|(?m:$))"),
      scope: vec![
        Scope {
            a: 47288689447403520,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259061529018368,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^--((?:[\\w=?:-]*[a-zA-Z0-9][\\w=?:-]*))(?:\\n|(?m:$))"),
      scope: vec![
        Scope {
            a: 47288620756172904,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259061529018368,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6994 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6994 }),
    ]),
      with_prototype: None
    }),
]
} }