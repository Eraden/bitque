
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
      regex: Regex::new("(?i)\\b(mov([auhl]|msk)pd)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 469225778383421440,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b((add|div|max|min|mul|sub|sqrt)[ps]d)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 469225701074010112,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b((andn?|x?or)pd)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 469219838443651072,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b((cmpp|u?comis)d)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 469225950182113280,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b((shuf|unpck[hl])pd)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 469225975951917056,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(cvt(dq2pd|pi2pd|ps2pd|pd2ps|si2sd|sd2ss|ss2sd|t?(pd2dq|pd2pi|sd2si)))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 469224266554933248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(cvt(dq2ps|ps2dq|tps2dq))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 469225984541851648,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(mov(dq[au]|q2dq|dq2q))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 469225722654294016,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(p((add|sub|(s[lr]l|mulu|unpck[hl]q)d)q|shuf(d|[hl]w)))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 469225722563592192,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(clflush|[lm]fence|pause|maskmovdqu|movnt(dq|i|pd))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 469225739728715776,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }