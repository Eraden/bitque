
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 4101 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4149 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)^\\s*(interface)\\s+(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576489594938,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130646081594,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4074 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)^\\s*(namespace)\\b\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787037044794,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4077 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)^\\s*(trait)\\s+(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576537763898,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130694250554,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4112 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*((?!default|else)\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*(:)(?!:)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130642149434,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620724912128,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{"),
      scope: vec![
        Scope {
            a: 47288521951477942,
            b: 16325548649218048,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\}"),
      scope: vec![
        Scope {
            a: 47288521951477931,
            b: 16325548649218048,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4110 })),
]
} }