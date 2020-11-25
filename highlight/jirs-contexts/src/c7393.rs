
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
        a: 844893081567232,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844893081567232,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\!"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7391 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\'"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7394 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\\")"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7392 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:(\\&) \\s* ([A-Za-z_][A-Za-z_0-9]*))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52639746252144640,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576614965357,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:([A-Za-z_][A-Za-z_0-9]*) \\s* (=) \\s* )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632122477,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52645535868059648,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("/"),
      scope: vec![
        Scope {
            a: 52641507188736000,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:\\.true\\.|\\.false\\.|\\bt\\b|\\bf\\b)"),
      scope: vec![
        Scope {
            a: 59955110644350976,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[0-9.]*\\b"),
      scope: vec![
        Scope {
            a: 59955089169514496,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:(?<!\\w)  [-+]?  (\\d+\\.\\d+ | \\d+\\. | \\.\\d+ | \\d+ )  ([ed][-+]?\\d+)?  (_\\w+)? )"),
      scope: vec![
        Scope {
            a: 59955089232494701,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }