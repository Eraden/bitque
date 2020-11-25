
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
      regex: Regex::new("(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629382021302,
            b: 14918173765664768,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3529 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[a-z][a-zA-Z0-9\'_]"),
      scope: vec![
        Scope {
            a: 59392130632450697,
            b: 48981305990643712,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bas\\b"),
      scope: vec![
        Scope {
            a: 52636787055067189,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("eof"),
      scope: vec![
        Scope {
            a: 59955110708445237,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("_"),
      scope: vec![
        Scope {
            a: 59955110707593269,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\[)(\\^?)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629365833910,
            b: 14918173765664768,
        },
    ]),(2, vec![
        Scope {
            a: 47288629365834568,
            b: 14918173765664768,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3530 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*|\\+|\\?"),
      scope: vec![
        Scope {
            a: 52636628141277357,
            b: 14918173765664768,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\|"),
      scope: vec![
        Scope {
            a: 52636628141278272,
            b: 14918173765664768,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3535 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3539 })),
]
} }