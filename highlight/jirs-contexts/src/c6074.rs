
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
      regex: Regex::new("(?i)\\b(fcmov(n?([beu]|be)))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 457966779420442624,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(f(i?(ld|stp?)|b(ld|stp)|xch))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 457966779329740800,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(f((add|div|mul|sub)p?|i(add|div|mul|sub)|(div|sub)rp?|i(div|sub)r))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 457966783612190720,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(f(prem1?|abs|chs|rndint|scale|sqrt|xtract))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 457966783624708096,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(f(u?com[ip]?p?|icomp?|tst|xam))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 457961131433000960,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(f(sin|cos|sincos|pa?tan|2xm1|yl2x(p1)?))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 457966787904929792,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(fld(1|z|pi|l2[et]|l[ng]2))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 457966792199897088,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(f((inc|dec)stp|free|n?(init|clex|st[cs]w|stenv|save)|ld(cw|env)|rstor|nop)|f?wait)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 457966796494864384,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(fx(save|rstor)(64)?)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 457966800789831680,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }