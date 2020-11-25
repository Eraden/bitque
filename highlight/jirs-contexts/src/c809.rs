
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
      regex: Regex::new("\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b"),
      scope: vec![
        Scope {
            a: 49258876839460864,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\)"),
      scope: vec![
        Scope {
            a: 47288521944400043,
            b: 3659174697238528,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 808 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(","),
      scope: vec![
        Scope {
            a: 47288620721963008,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 707 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 805 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 791 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\.\\.\\."),
      scope: vec![
        Scope {
            a: 52636628128366605,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(/\\*).*(\\*/)"),
      scope: vec![
        Scope {
            a: 51510878516609024,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323038733,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323038733,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?m:$)\\n"),
      scope: vec![
        Scope {
            a: 50103314684117005,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }