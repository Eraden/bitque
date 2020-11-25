
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
      regex: Regex::new("\\)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629327560875,
            b: 12103423998558208,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7801 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7806 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\.\\.\\."),
      scope: vec![
        Scope {
            a: 52636628152877099,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([_$a-zA-Z][$\\w]*)"),
      scope: vec![
        Scope {
            a: 49258876850208811,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(","),
      scope: vec![
        Scope {
            a: 47288620732645553,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("="),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 52636628111130667,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7752 }),
    ]),
      with_prototype: None
    }),
]
} }