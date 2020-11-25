
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
      regex: Regex::new("\\$\\$"),
      scope: vec![
        Scope {
            a: 55451536836526117,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 10414574138294272,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2763 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\\\\\[)"),
      scope: vec![
        Scope {
            a: 55451536836526117,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 10414574138294272,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2764 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n((\\\\)begin)(\\{)\\s*((?:\n  align|alignat|aligned|alignedat|displaymath\n  |eqnarray|equation|flalign|gather|gathered\n  |math|multline|x?xalignat|split\n  |dmath|dseries|dgroup|darray|dsuspend\n)\\*?)\\s*(\\})"),
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
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2765 }),
    ]),
      with_prototype: None
    }),
]
} }