
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
      regex: Regex::new("\\b(namespace|module)\\s*(public|internal|private)?\\s+([\\p{L}][\\p{L}0-9\'_. ]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636271616458752,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636271616458752,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130630090857,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7044 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(open)\\s+([\\p{L}][\\p{L}0-9\'_]*)(?=(\\.[A-Z][\\p{L}0-9_]*)*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636271616458752,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630090857,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7045 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(module)\\s+([A-Z][\\p{L}0-9\'_]*)\\s*(=)\\s*([A-Z][\\p{L}0-9\'_]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636271616458752,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632450422,
            b: 29554872554618880,
        },
    ]),(3, vec![
        Scope {
            a: 47288620866207849,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130630090857,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7046 }),
    ]),
      with_prototype: None
    }),
]
} }