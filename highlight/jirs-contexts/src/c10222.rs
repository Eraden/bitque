
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
      regex: Regex::new("(\\])(?:(:)(\\d{1,5}))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956395,
            b: 44473046320283648,
        },
    ]),(2, vec![
        Scope {
            a: 47288620756172958,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955089176461530,
            b: 44473046320283648,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("!"),
      scope: vec![
        Scope {
            a: 52636628114800798,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10240 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10223 }),
    ]),
      with_prototype: None
    }),
]
} }