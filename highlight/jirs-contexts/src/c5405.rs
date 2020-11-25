
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
      regex: Regex::new("(%)([%+-])"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46443865171559831,
            b: 20829148276588544,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322515863,
            b: 20829148276588544,
        },
    ]),(2, vec![
        Scope {
            a: 49259061615984714,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(%)(\\d+)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46443865171559831,
            b: 20829148276588544,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322515863,
            b: 20829148276588544,
        },
    ]),(2, vec![
        Scope {
            a: 59955089176461530,
            b: 402791009500528640,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(%)(\\??)(\\w+)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46443865171559831,
            b: 20829148276588544,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322515863,
            b: 20829148276588544,
        },
    ]),(2, vec![
        Scope {
            a: 52636628101628751,
            b: 20829148276588544,
        },
    ]),(3, vec![
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
      regex: Regex::new("%"),
      scope: vec![
        Scope {
            a: 46443865171559831,
            b: 20829148276588544,
        },
        Scope {
            a: 47288629322515863,
            b: 20829148276588544,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }