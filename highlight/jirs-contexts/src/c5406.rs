
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
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$)(\\{)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46443865171558576,
            b: 20829148276588544,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322514506,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288522029924528,
            b: 51228763588919296,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5345 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$)(\\d)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46443865171558576,
            b: 20829148276588544,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322514506,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087310291018,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$)([$#@!~*?_-])(?!\\w)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46443865171558576,
            b: 20829148276588544,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322514506,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259061527052288,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$)([\\p{L}_][\\p{L}\\p{N}_]*)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46443865171558576,
            b: 20829148276588544,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322514506,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087310291018,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }