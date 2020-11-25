
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
        a: 844652563398656,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844652563398656,
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
      regex: Regex::new("^\\s*({)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521949642804,
            b: 51228673394606080,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3525 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(let)\\s+([a-z][a-zA-Z0-9\'_]*)\\s+="),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787083444277,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632450697,
            b: 305400577364328448,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3526 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(rule|and)\\s+([a-z][a-zA-Z0-9_]*)\\s+(=)\\s+(parse)(?=\\s*(?m:$))|((?<!\\|)(\\|)(?!\\|))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787016007680,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630616126,
            b: 14918173765664768,
        },
    ]),(3, vec![
        Scope {
            a: 52636628102217728,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636787016007680,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288620791562293,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3527 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3539 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3536 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("="),
      scope: vec![
        Scope {
            a: 52636628137082933,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3528 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(’|‘|“|”)"),
      scope: vec![
        Scope {
            a: 50103314723962933,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }