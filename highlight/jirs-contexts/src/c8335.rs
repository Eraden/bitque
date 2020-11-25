
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
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(SYNOPSIS|SYNTAX|SINTASSI|SKŁADNIA|СИНТАКСИС|書式)"),
      scope: vec![
        Scope {
            a: 114281636568236032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 777 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 8332 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\S.*(?m:$)"),
      scope: vec![
        Scope {
            a: 114281636568236032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b([A-Za-z0-9_\\-]+)(\\()([^)]*)(\\))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130630615040,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628098744320,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955089162371072,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636628098744320,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:[^a-zA-Z0-9_-]|^|\\s)(--?[A-Za-z0-9][A-Za-z0-9-]*)(?:(=)?(\"?)([A-Za-z0-9]+)(\"?))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130619015168,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628098744320,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 49258876838608896,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323956395,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }