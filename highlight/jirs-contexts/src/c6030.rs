
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
        a: 46444466403475541,
        b: 24488322973827072,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444466403475541,
        b: 24488322973827072,
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
    Pattern::Include(ContextReference::Named("preprocessor-comments".to_string())),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(?:[\\p{L}_?](?:[\\p{L}\\p{N}_$#@~.?]*)))(?=\\()"),
      scope: vec![
        Scope {
            a: 59392130630615295,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6031 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:[\\p{L}_?](?:[\\p{L}\\p{N}_$#@~.?]*))"),
      scope: vec![
        Scope {
            a: 59392130632974591,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6115 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6066 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6103 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6067 })),
]
} }