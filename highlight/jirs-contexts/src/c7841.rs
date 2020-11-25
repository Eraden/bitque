
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
      regex: Regex::new("([_$a-zA-Z][$\\w]*)(\\.)(prototype)\\s*(=)\\s*"),
      scope: vec![
        Scope {
            a: 46446914541781035,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632319019,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628113883179,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49259061576269867,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636628111130667,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([_$a-zA-Z][$\\w]*)(\\.)(prototype)\\b"),
      scope: vec![
        Scope {
            a: 46446914530705451,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632319019,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628113883179,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49259061576269867,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }