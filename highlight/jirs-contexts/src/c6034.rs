
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
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:[\\p{L}_?](?:[\\p{L}\\p{N}_$#@~.?]*)))(\\:)(?=\\w+)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466379489367,
            b: 0,
        },
        Scope {
            a: 49258876868165717,
            b: 24488322973827072,
        },
    ]),(2, vec![
        Scope {
            a: 47288620726681687,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\,"),
      scope: vec![
        Scope {
            a: 47288620726681687,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6059 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\w+\\b"),
      scope: vec![
        Scope {
            a: 50103314664915149,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6066 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6067 })),
]
} }