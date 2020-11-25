
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
      regex: Regex::new("(?i:Is\\s+Not\\b)"),
      scope: vec![
        Scope {
            a: 50103314665439236,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:[+*^&/\\\\-]|\\b(?i:Mod)\\b)\\s*="),
      scope: vec![
        Scope {
            a: 50103314665439236,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("><"),
      scope: vec![
        Scope {
            a: 50103314665439236,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:[+*^&/\\\\-]|\\b(?i:Mod)\\b)|[=><])|(\\b(?i:And|Not|Or|Xor|Is)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628099006464,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628114800644,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 50 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(":"),
      scope: vec![
        Scope {
            a: 47288689454284804,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\.)(\\.)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788226932740,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 50103314665439236,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 48 }),
    ]),
      with_prototype: None
    }),
]
} }