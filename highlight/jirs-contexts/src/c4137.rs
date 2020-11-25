
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
      regex: Regex::new("(->)(\\$?\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788252622906,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629322514490,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4064 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(->)(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)(?=\\s*\\()"),
      scope: vec![
        Scope {
            a: 46444882996625466,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788252622906,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49258881137377280,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4115 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4116 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(->)((\\$+)?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788252622906,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087306883130,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629322514490,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4098 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(::)(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788251050042,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955136420315194,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4098 }),
    ]),
      with_prototype: None
    }),
]
} }