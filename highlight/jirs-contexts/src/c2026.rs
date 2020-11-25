
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
        a: 844566664052736,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844566664052736,
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
      regex: Regex::new("^(#!).+(?m:$)\\n"),
      scope: vec![
        Scope {
            a: 51510711060725793,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323038753,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(package)\\s+([^ ;]+)"),
      scope: vec![
        Scope {
            a: 46446411996266496,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787058933793,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576508928033,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(import)\\s+([^ ;$]+);?"),
      scope: vec![
        Scope {
            a: 46445256650063872,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787041304609,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576491298849,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2020 })),
]
} }