
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
      regex: Regex::new("(?xi)\n      (\\.(?:and | or | not)\\.)\n    | (//)\n    | (=>)\n    | (/= | == | >= | <= | < | >)\n    | (\\*\\* | \\* | \\+ | - | / )\n    | (\\.(?:eq|lt|le|gt|ge|ne|eqv|neqv)\\.)\n"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628114802885,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628250527941,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636628250593477,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636628119259333,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 52636628119193797,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 52636628119259333,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\:\\:)"),
      scope: vec![
        Scope {
            a: 52636628124960965,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:=)"),
      scope: vec![
        Scope {
            a: 52636628111132869,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:=)"),
      scope: vec![
        Scope {
            a: 52636628111132869,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\%"),
      scope: vec![
        Scope {
            a: 52636628250659013,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }