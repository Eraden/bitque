
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
        index: 8656,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(optional|required|repeated)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439032225792,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8637 }),
        ContextReference::Direct(ContextId { index: 8638 }),
        ContextReference::Direct(ContextId { index: 8645 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(map)\\b\\s*\\b(<)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925375391170692,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629357576374,
            b: 37154696925806592,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8637 }),
        ContextReference::Direct(ContextId { index: 8638 }),
        ContextReference::Direct(ContextId { index: 8642 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\b([A-Za-z][A-Za-z0-9_]*)\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8637 }),
        ContextReference::Direct(ContextId { index: 8638 }),
        ContextReference::Direct(ContextId { index: 8645 }),
    ]),
      with_prototype: None
    }),
]
} }