
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
  meta_include_prototype: false,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 5301 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5302 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=[\\}\\)\\]])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\?>"),
      scope: vec![
        Scope {
            a: 47288629324153003,
            b: 22517998136852480,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("/>"),
      scope: vec![
        Scope {
            a: 47288629324153003,
            b: 22517998136852480,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(">"),
      scope: vec![
        Scope {
            a: 47288629324153003,
            b: 22517998136852480,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5304 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\s+|^)(?:([\\p{L}_][\\p{L}\\p{N}_.-]*)(:))?([\\p{L}_][\\p{L}\\p{N}_.-]*)\\s*(=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392186477183350,
            b: 22517998136852480,
        },
    ]),(2, vec![
        Scope {
            a: 59392186477183056,
            b: 0,
        },
        Scope {
            a: 47288620745621584,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392186477184391,
            b: 22517998136852480,
        },
    ]),(4, vec![
        Scope {
            a: 47288620737429584,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5300 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\s+|^)([\\p{L}\\p{N}:_.-]+)\\s*(=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314746441808,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620737429584,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5305 })),
]
} }