
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
      regex: Regex::new("(?:-)?(\\])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636691562743,
            b: 48132538845102080,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\-"),
      scope: vec![
        Scope {
            a: 52636628119322698,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\.)[[:word:]](\\.)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620814958774,
            b: 20829148276588544,
        },
    ]),(2, vec![
        Scope {
            a: 47288620814958763,
            b: 20829148276588544,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(=)[[:word:]](=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620815024310,
            b: 20829148276588544,
        },
    ]),(2, vec![
        Scope {
            a: 47288620815024299,
            b: 20829148276588544,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(:)[\\p{Ll}]+(:)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620775899318,
            b: 20829148276588544,
        },
    ]),(2, vec![
        Scope {
            a: 47288620775899307,
            b: 20829148276588544,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\[(?![\\.=:])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5401 })),
]
} }