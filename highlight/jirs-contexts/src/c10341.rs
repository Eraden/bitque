
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 10338 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(interface)\\s+(\\b([a-z](\\-*[a-z0-9])*(\\.[a-z0-9](\\-*[a-z0-9])*)+)\\b)"),
      scope: vec![
        Scope {
            a: 46444251662057636,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576489595044,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130646081700,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(method)\\s+(\\b([A-Z][a-zA-Z0-9_]*)\\b)\\s*(?=[(])"),
      scope: vec![
        Scope {
            a: 46444084158333092,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576473407652,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130629894308,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10340 }),
        ContextReference::Direct(ContextId { index: 10336 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(type)\\s+(\\b([A-Z][a-zA-Z0-9_]*)\\b)\\s*(?=[(])"),
      scope: vec![
        Scope {
            a: 46444251662057636,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475963556,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632450212,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10336 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(error)\\s+(\\b([A-Z][a-zA-Z0-9_]*)\\b)\\s*(?=[(])"),
      scope: vec![
        Scope {
            a: 46444526539964580,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576480157860,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130636644516,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10336 }),
    ]),
      with_prototype: None
    }),
]
} }