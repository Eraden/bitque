
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
        a: 844897376534528,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844897376534528,
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
      regex: Regex::new("(?xi:^ (Error:) \\s (.*) )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636975998304256,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615150,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^ (Fatal\\sError:) \\s (.*) )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636975998304256,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615150,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^ [/]  .*  \\.(f|for|fpp|f\\d\\d):\\d+:\\d+:)"),
      scope: vec![
        Scope {
            a: 49258876867313774,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^compilation terminated."),
      scope: vec![
        Scope {
            a: 50112540250603520,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\[.*"),
      scope: vec![
        Scope {
            a: 51519919429124096,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }