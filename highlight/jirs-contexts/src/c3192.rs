
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
        a: 46444131379839024,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444131379839024,
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
      regex: Regex::new("^\\t([-@]{1,2})?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955110640353280,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3193 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^([ ]+)([-@]{1,2})?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314714460531,
            b: 261771934499340288,
        },
    ]),(2, vec![
        Scope {
            a: 59955110640353280,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3188 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(?!\\t)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }