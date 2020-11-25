
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
        a: 46445102034649175,
        b: 262616153271042048,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46445102034649175,
        b: 262616153271042048,
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
            a: 49259087297577047,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6054 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\w"),
      scope: vec![
        Scope {
            a: 50103314665111955,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }