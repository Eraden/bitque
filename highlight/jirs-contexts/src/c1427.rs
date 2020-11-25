
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
        index: 1627,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 1582 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("=>|\\:="),
      scope: vec![
        Scope {
            a: 46445780635222016,
            b: 0,
        },
        Scope {
            a: 47288620757876985,
            b: 5629499534213120,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1428 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(",?"),
      scope: vec![
        Scope {
            a: 46445780635222016,
            b: 0,
        },
        Scope {
            a: 47288620757877339,
            b: 5629499534213120,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1429 }),
    ]),
      with_prototype: None
    }),
]
} }