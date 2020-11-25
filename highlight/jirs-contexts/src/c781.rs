
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
      regex: Regex::new("(?x:\n  \\b\\d+\n  (?:\n    (?: (\\.) (?: \\d+ (?:[eE][-+]??\\d+)? | (?:[eE][-+]??\\d+) | (?=[^.])) | (?:[eE][-+]??\\d+) )\n    (?: ([fFlL])\\b | ([a-zA-Z_][\\p{L}\\p{N}_]*)? ) | ([fF])\\b\n  )\n  | (\\.) \\d+ (?:[eE][-+]??\\d+)? (?: ([fFlL])\\b | ([a-zA-Z_][\\p{L}\\p{N}_]*)? )\n)"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 3659174697238528,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397901,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553229,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 50103314667667928,
            b: 3659174697238528,
        },
    ]),(4, vec![
        Scope {
            a: 48414576476553229,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288620735397901,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 48414576476553229,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 50103314667667928,
            b: 3659174697238528,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b0[xX](?=[\\p{L}\\p{N}.]+?[pP])"),
      scope: vec![
        Scope {
            a: 47288629325070764,
            b: 3659174697238528,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 695 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b0[xX]"),
      scope: vec![
        Scope {
            a: 47288629325070764,
            b: 3659174697238528,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 696 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b0(?=\\d)"),
      scope: vec![
        Scope {
            a: 47288629325070764,
            b: 3659174697238528,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 697 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b\\d+"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 698 }),
    ]),
      with_prototype: None
    }),
]
} }