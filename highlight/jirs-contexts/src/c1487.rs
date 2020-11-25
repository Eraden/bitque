
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 1669 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[a-z][_A-Za-z\\d@]*"),
      scope: vec![
        Scope {
            a: 46445879419469824,
            b: 0,
        },
        Scope {
            a: 59392130659123220,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\'"),
      scope: vec![
        Scope {
            a: 47288629349318838,
            b: 5629499534213120,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1488 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1580 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1518 })),
]
} }