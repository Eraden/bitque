
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
        a: 46446119994327077,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46446119994327077,
        b: 0,
    },
],
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
      regex: Regex::new("((\\\\)end)(\\{)(tabular)(\\})"),
      scope: vec![
        Scope {
            a: 46446119994327077,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255096762405,
            b: 0,
        },
        Scope {
            a: 52636636701196459,
            b: 10414574138294272,
        },
    ]),(2, vec![
        Scope {
            a: 47288629362032677,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(4, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2968 })),
]
} }