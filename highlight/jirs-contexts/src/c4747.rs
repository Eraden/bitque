
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
      regex: Regex::new("\\\\[bBAZzG><]|[\\^$]"),
      scope: vec![
        Scope {
            a: 52636636776497196,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4774 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[QEK]"),
      scope: vec![
        Scope {
            a: 52636636691562496,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4774 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[kg](?:(<)([^>]+)(>)|(\')([^\']+)(\')|(\\{)([^}]+)(\\})|(-?\\d+))"),
      scope: vec![
        Scope {
            a: 52636787100418092,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629398995126,
            b: 12384898975268864,
        },
    ]),(2, vec![
        Scope {
            a: 49259087379890220,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629398995115,
            b: 12384898975268864,
        },
    ]),(4, vec![
        Scope {
            a: 47288629398995126,
            b: 12384898975268864,
        },
    ]),(5, vec![
        Scope {
            a: 49259087379890220,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 47288629398995115,
            b: 12384898975268864,
        },
    ]),(7, vec![
        Scope {
            a: 47288629398995126,
            b: 12384898975268864,
        },
    ]),(8, vec![
        Scope {
            a: 49259087379890220,
            b: 0,
        },
    ]),(9, vec![
        Scope {
            a: 47288629398995115,
            b: 12384898975268864,
        },
    ]),(10, vec![
        Scope {
            a: 49259087379890220,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\([1-9]\\d*)"),
      scope: vec![
        Scope {
            a: 52636787100418092,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087379890220,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }