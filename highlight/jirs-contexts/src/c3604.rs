
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
      regex: Regex::new("\\btemplate\\b"),
      scope: vec![
        Scope {
            a: 48414576491495480,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(::)\\s*)?(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*(<)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788251050040,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49258881137246208,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288521955803318,
            b: 15762598695796736,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3605 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(::)\\s*)?(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788251050040,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49258881137246208,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288521944400054,
            b: 15762598695796736,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3606 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(::)\\s*)?(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788251050040,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49258881137246208,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288521944400054,
            b: 15762598695796736,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3607 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 604 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3703 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 46443865082167296,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 15762598695796736,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3608 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{"),
      scope: vec![
        Scope {
            a: 46443865082167296,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 15762598695796736,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3609 }),
    ]),
      with_prototype: None
    }),
]
} }