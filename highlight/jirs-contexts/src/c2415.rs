
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
  prototype: Some(
    ContextId {
        index: 2422,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@)param\\b"),
      scope: vec![
        Scope {
            a: 52636787039011625,
            b: 11540474045136896,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323300905,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2414 }),
        ContextReference::Direct(ContextId { index: 2421 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@)see\\b"),
      scope: vec![
        Scope {
            a: 52636787039011626,
            b: 11540474045136896,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323300905,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2414 }),
        ContextReference::Direct(ContextId { index: 2425 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@)(throws|exception)\\b"),
      scope: vec![
        Scope {
            a: 52636787039011550,
            b: 11540474045136896,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323300905,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2414 }),
        ContextReference::Direct(ContextId { index: 2423 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@)(uses|provides)\\b"),
      scope: vec![
        Scope {
            a: 52636787039011627,
            b: 11540474045136896,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323300905,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2414 }),
        ContextReference::Direct(ContextId { index: 2423 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@)(return|deprecated|author|version|since|apiNote|impl(?:Note|Spec)|moduleGraph|serial(?:Field|Data)?)\\b"),
      scope: vec![
        Scope {
            a: 52636787039010857,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323300905,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2414 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("@"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }