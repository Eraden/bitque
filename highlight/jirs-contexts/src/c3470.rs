
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
      regex: Regex::new("\\b(end)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636743073844,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(method)\\s+(virtual\\s+)?(private\\s+)?([a-z_][a-zA-Z0-9\'_]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787079905332,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787079905332,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636787079970868,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130630615206,
            b: 14636698788954112,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3471 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(constraint)\\s+([a-z_\'][a-zA-Z0-9\'_]*)\\s+(=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787026886708,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576465936384,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636628102152192,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3472 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3514 })),
]
} }