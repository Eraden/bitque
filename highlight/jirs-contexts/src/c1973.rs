
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
        a: 844562381864965,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844562381864965,
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
      regex: Regex::new("((>)?\\s*)(>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 844562381864965,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444230150717440,
            b: 0,
        },
        Scope {
            a: 47288629324153003,
            b: 1407374883553280,
        },
    ]),(3, vec![
        Scope {
            a: 47288521949642923,
            b: 9007199254740992,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1974 }),
        ContextReference::Direct(ContextId { index: 2107 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 1975 }),
    )
    }),
]
} }