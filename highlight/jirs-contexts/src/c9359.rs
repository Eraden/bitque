
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
      regex: Regex::new("\\}"),
      scope: vec![
        Scope {
            a: 47288521962160299,
            b: 41939771529887744,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9385 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9373 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(\\b(?!null|false|true)[\\p{L}][\\p{L}\\p{N}_-]*\\b)\\s*(\\=)\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445780653244565,
            b: 0,
        },
        Scope {
            a: 55451949106266112,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628108509184,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9386 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\\").*(\\\"))\\s*(\\=)\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445780653244565,
            b: 0,
        },
        Scope {
            a: 55451420828565653,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956406,
            b: 41939771529887744,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323956395,
            b: 41939771529887744,
        },
    ]),(4, vec![
        Scope {
            a: 52636628108509184,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9386 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 47288521948004534,
            b: 41939771529887744,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9360 }),
    ]),
      with_prototype: None
    }),
]
} }