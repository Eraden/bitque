
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
        index: 6125,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\bstruc\\b"),
      scope: vec![
        Scope {
            a: 61925255091126359,
            b: 0,
        },
        Scope {
            a: 46445102034649175,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6051 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\bendstruc\\b"),
      scope: vec![
        Scope {
            a: 50103314668060849,
            b: 23925746682560512,
        },
        Scope {
            a: 46445102034649175,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\bistruc\\b"),
      scope: vec![
        Scope {
            a: 61925255091126359,
            b: 0,
        },
        Scope {
            a: 46445102034649175,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6053 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\biend\\b"),
      scope: vec![
        Scope {
            a: 50103314668060849,
            b: 23925746682560512,
        },
        Scope {
            a: 46445102034649175,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\balignb?\\b"),
      scope: vec![
        Scope {
            a: 61925255091126359,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(sectalign)\\s+(on|off)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255091126359,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 61925409709949015,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }