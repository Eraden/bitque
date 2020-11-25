
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
        a: 46444466379489367,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444466379489367,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: Some(
    ContextId {
        index: 6125,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(?:[\\p{L}_?](?:[\\p{L}\\p{N}_$#@~.?]*)))(?=\\()"),
      scope: vec![
        Scope {
            a: 59392130632974591,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6029 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:[\\p{L}_?](?:[\\p{L}\\p{N}_$#@~.?]*))\\b"),
      scope: vec![
        Scope {
            a: 46444466379489367,
            b: 0,
        },
        Scope {
            a: 59392130632974591,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6066 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6103 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6067 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6068 })),
]
} }