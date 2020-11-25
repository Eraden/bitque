
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
        index: 4695,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\\\)(?:code))(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255167344705,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629362032705,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 51228724934213632,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4682 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\\\)(?:Sexpr))(?:(\\[)(?:[^\\]]*)(\\]))?(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255171932225,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629362032705,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318583141,
            b: 51228724934213632,
        },
    ]),(4, vec![
        Scope {
            a: 47288629318583141,
            b: 48132500190396416,
        },
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228724934213632,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4683 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\\\)(?:usage))(\\{)(?:\\n)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255171997761,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629362032705,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 51228724934213632,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4684 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\\\)(?:examples))(\\{)(?:\\n)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255172063297,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629362032705,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 51228724934213632,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4685 }),
    ]),
      with_prototype: None
    }),
]
} }