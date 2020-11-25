
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
        index: 2981,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\\\\\\\)(?:(\\[)\\s*-?((?:[\\p{Nd}]|\\.)*)\\s*(\\w*)\\s*(\\]))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955200854261797,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629318583141,
            b: 51229936117415936,
        },
    ]),(3, vec![
        Scope {
            a: 59955089185112101,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636787035275301,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629318583141,
            b: 51229936117415936,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3006 })),
]
} }