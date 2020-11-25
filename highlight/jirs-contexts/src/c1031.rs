
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
      regex: Regex::new("(?x:\n  [-+]? \\d* (\\.) \\d+ (?:[eE](?:[-+]?\\d+))?\n| [-+]? \\d+          (?:[eE](?:[-+]?\\d+))\n)((?i:dpi|dpcm|dppx))\\b"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 3940649673949184,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397902,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787033309198,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:[-+]?\\d+)((?i:dpi|dpcm|dppx))\\b"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 3940649673949184,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787033309198,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }