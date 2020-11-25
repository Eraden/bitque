
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
        index: 5435,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 5387 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 47288629403910326,
            b: 20829148276588544,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5326 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5696 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\blet\\b"),
      scope: vec![
        Scope {
            a: 61925255179141195,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5327 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\[\\[)(?=\\s)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255179206838,
            b: 20829148276588544,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5394 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\[)(?=\\s)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255178485942,
            b: 20829148276588544,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5392 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(?=\\s)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629403910530,
            b: 51228763588919296,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5328 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5381 }),
        ContextReference::Direct(ContextId { index: 5388 }),
    ]),
      with_prototype: None
    }),
]
} }