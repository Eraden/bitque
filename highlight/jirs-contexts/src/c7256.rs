
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 7257 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)([^\\n\\S]+)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445029029708509,
            b: 29836347531329536,
        },
    ]),(2, vec![
        Scope {
            a: 46445029029708509,
            b: 378021348988944384,
        },
    ]),(3, vec![
        Scope {
            a: 46445029029708509,
            b: 29836347531329536,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{\\}"),
      scope: vec![
        Scope {
            a: 46445029029708509,
            b: 29836347531329536,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{(?=[^,$\\{]+(?:\\n|\\\\\\n|\\}))"),
      scope: vec![
        Scope {
            a: 46445029029710009,
            b: 29836347531329536,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7209 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{"),
      scope: vec![
        Scope {
            a: 46445029161042026,
            b: 0,
        },
        Scope {
            a: 47288521962160310,
            b: 29836347531329536,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7213 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\*\\*)|(\\*)|(\\?)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46452991890948096,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52636628245610602,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628245676138,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636628245741674,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }