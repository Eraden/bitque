
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
    Scope {
        a: 52636628115456085,
        b: 24488322973827072,
    },
    Scope {
        a: 49258876855321216,
        b: 23925746682560512,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444466379489367,
        b: 0,
    },
    Scope {
        a: 52636628115456085,
        b: 24488322973827072,
    },
    Scope {
        a: 49258876855321216,
        b: 23925746682560512,
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
      regex: Regex::new("(?:[\\p{L}_?](?:[\\p{L}\\p{N}_$#@~.?]*))"),
      scope: vec![
        Scope {
            a: 49258876855321216,
            b: 90353832602042368,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=`|\'|\")"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6043 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6102 })),
]
} }