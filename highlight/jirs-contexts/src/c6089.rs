
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
      regex: Regex::new("(?i)\\b(mov(([ahlu]|hl|lh|msk)ps|ss))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 470914628243685376,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b((add|div|max|min|mul|rcp|r?sqrt|sub)[ps]s)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 470914550934274048,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(cmp[ps]s|u?comiss)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 470908980361691136,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b((andn?|x?or)ps)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 470908688303915008,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b((shuf|unpck[hl])ps)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 470914825812180992,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(cvt(pi2ps|si2ss|ps2pi|tps2pi|ss2si|tss2si))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 470913116415197184,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b((ld|st)mxcsr)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 470914649718521856,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(p(avg[bw]|extrw|insrw|(max|min)(sw|ub)|sadbw|shufw|mulhuw|movmskb))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 470914572409110528,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(maskmovq|movntps|sfence)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 470914589588979712,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(prefetch(nta|t[0-2]|w(t1)?))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 470914830107148288,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }