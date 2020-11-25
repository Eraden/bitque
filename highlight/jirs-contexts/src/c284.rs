
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
        a: 46445024720388096,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46445024720388096,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: Some(
    ContextId {
        index: 399,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\[)(,*)(\\]))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444990360649728,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288521961570486,
            b: 3096224743817216,
        },
    ]),(3, vec![
        Scope {
            a: 47288620721831936,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288521961570475,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\["),
      scope: vec![
        Scope {
            a: 47288521961570486,
            b: 3096224743817216,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 285 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\s*((\\()\\s*(\\)))\\s*)?(\\{)"),
      scope: vec![
        Scope {
            a: 46445024720388096,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46443865079218176,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288521944400054,
            b: 3096224743817216,
        },
    ]),(3, vec![
        Scope {
            a: 47288521944400043,
            b: 3096224743817216,
        },
    ]),(4, vec![
        Scope {
            a: 46445029015355392,
            b: 0,
        },
        Scope {
            a: 47288521962160310,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 367 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 46443865079218176,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 3096224743817216,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 342 }),
        ContextReference::Direct(ContextId { index: 329 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 414 })),
]
} }