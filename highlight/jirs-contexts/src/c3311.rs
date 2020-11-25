
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 3341 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\*\\*)(\\*)(?=\\S)(?!\\*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629354365110,
            b: 13792273858822144,
        },
    ]),(2, vec![
        Scope {
            a: 114282585759219712,
            b: 0,
        },
        Scope {
            a: 47288629368193206,
            b: 13792273858822144,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3211 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*\\*(?=\\S)(?!\\*\\*|\\*\\s)"),
      scope: vec![
        Scope {
            a: 47288629354365110,
            b: 13792273858822144,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3214 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(__)(_)(?=\\S)(?!_)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629354365110,
            b: 13792273858822144,
        },
    ]),(2, vec![
        Scope {
            a: 114282585759219712,
            b: 0,
        },
        Scope {
            a: 47288629368193206,
            b: 13792273858822144,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3215 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b__(?=\\S)(?!_[_\\s])"),
      scope: vec![
        Scope {
            a: 47288629354365110,
            b: 13792273858822144,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3218 }),
    ]),
      with_prototype: None
    }),
]
} }