
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 3010 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(\n  (\\\\)\n  (?:(?:re)?newcommand\\*?)\n)\n(?:\n  (\\{)(\\\\[A-Za-z@]+)(\\})\n  | (\\\\[A-Za-z@])+\n)\n(?:(\\[)(?:[^\\]]*)(\\]))*\n(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255143161893,
            b: 0,
        },
        Scope {
            a: 48414439081181221,
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
            a: 59392130676621349,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ]),(6, vec![
        Scope {
            a: 61925255087980544,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 47288629318583141,
            b: 51228604675129344,
        },
    ]),(8, vec![
        Scope {
            a: 47288629318583141,
            b: 48132379931312128,
        },
    ]),(9, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2784 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(\n  (\\\\)\n  DeclareMathOperator\\*?\n)\n(?:\n  (\\{)(\\\\[A-Za-z@]+)(\\})\n  | (\\\\[A-Za-z@])+\n)\n(?:(\\[)(?:[^\\]]*)(\\]))?\n(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255143227429,
            b: 0,
        },
        Scope {
            a: 48414439081246757,
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
            a: 59392130676686885,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ]),(6, vec![
        Scope {
            a: 61925255087980544,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 47288629318583141,
            b: 51228604675129344,
        },
    ]),(8, vec![
        Scope {
            a: 47288629318583141,
            b: 48132379931312128,
        },
    ]),(9, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2785 }),
    ]),
      with_prototype: None
    }),
]
} }