
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
      regex: Regex::new("(?x)\n(\n  (\\\\)newcolumntype\n)\n(?:\n  (\\{)\n  (\n    (?:\\\\[A-Za-z@]+)\n    | (?:[A-Za-z@])\n  )\n  (\\})\n)\n(?:(\\[)(?:[^\\]]*)(\\]))*\n(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255143489573,
            b: 0,
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
            a: 61925255087980544,
            b: 0,
        },
        Scope {
            a: 59392130676949029,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ]),(6, vec![
        Scope {
            a: 47288629318583141,
            b: 51228604675129344,
        },
    ]),(7, vec![
        Scope {
            a: 47288629318583141,
            b: 48132379931312128,
        },
    ]),(8, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2882 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n((\\\\)begin)(\\{)(tabular)(\\})\n(?:(\\[)(?:b|c|t)(\\]))?\n(?=\\s*\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255097483301,
            b: 0,
        },
        Scope {
            a: 52636636701196470,
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
    ]),(6, vec![
        Scope {
            a: 47288629318583141,
            b: 51228604675129344,
        },
    ]),(7, vec![
        Scope {
            a: 47288629318583141,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2883 }),
    ]),
      with_prototype: None
    }),
]
} }