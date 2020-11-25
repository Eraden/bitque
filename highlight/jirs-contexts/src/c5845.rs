
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
        index: 5884,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(YAML)[ \\t]+(\\d+\\.\\d+)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787052118097,
            b: 22799473113563136,
        },
    ]),(2, vec![
        Scope {
            a: 59955089259954257,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5866 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(TAG)\n(?:[ \\t]+\n ((?:!(?:[0-9A-Za-z\\-]*!)?))\n (?:[ \\t]+ ((?x:\n    !              (?x: %[0-9A-Fa-f]{2} | [0-9A-Za-z\\-#;/?:@&=+$,_.!~*\'()\\[\\]] )*\n  | (?![,!\\[\\]{}]) (?x: %[0-9A-Fa-f]{2} | [0-9A-Za-z\\-#;/?:@&=+$,_.!~*\'()\\[\\]] )+\n)) )?\n)?\n"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787052118216,
            b: 22799473113563136,
        },
    ]),(2, vec![
        Scope {
            a: 48414576560177233,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925375442354257,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5866 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x) (\\w+) (?:[ \\t]+ (\\w+) (?:[ \\t]+ (\\w+))? )?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925461283571156,
            b: 22799473113563136,
        },
    ]),(2, vec![
        Scope {
            a: 55451949194346577,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 55451949194412113,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5866 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5866 }),
    ]),
      with_prototype: None
    }),
]
} }