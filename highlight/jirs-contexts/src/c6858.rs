
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
        a: 844850131894272,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844850131894272,
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
      regex: Regex::new("\\b(fn)\\b(?!.*->)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636695166976,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6779 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(fn)\\b(?=.*->)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636695166976,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6780 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6845 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(?=.*->)((?![^\"\']*(\"|\'\')[^\"\']*->)|(?=.*->[^\"\']*(\"|\'\')[^\"\']*->))((?!.*\\([^\\)]*->)|(?=[^\\(\\)]*->)|(?=\\s*\\(.*\\).*->))((?!.*\\b(fn)\\b)|(?=.*->.*\\bfn\\b))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636695166976,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6781 }),
    ]),
      with_prototype: None
    }),
]
} }