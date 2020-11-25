
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
        index: 6125,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(fisttp|lddqu|(addsub|h(add|sub))p[sd]|mov(sh|sl|d)dup|monitor|mwait)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 472033535943770112,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(ph(add|sub)(s?w|d))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 472040743004274688,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(p((abs|sign)[bdw]|maddubsw|mulhrsw|shufb|alignr))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 472040742913638400,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }