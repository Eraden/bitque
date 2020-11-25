
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
      regex: Regex::new("(?i)\\b(bl([cs](fill|ic?|msk)|cs)|t1mskc|tzmsk)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 454026305842511872,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(clgi|int3|invlpga|iretw|skinit|stgi|vm(load|mcall|run|save)|monitorx|mwaitx)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 454026305845002240,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b([ls]lwpcb|lwp(ins|val))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 454026305845067776,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(movnts[ds])\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 454026305845133312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(prefetch|clzero)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 454026305845198848,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b((extr|insert)q)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 454026305844740850,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vfn?m((add|sub)[ps][ds])|vfm((addsub|subadd)p[ds]))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 454026305845264384,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vp(cmov|(comu?|rot|sh[al])[bdqw]|mac(s?s(d(d|q[hl])|w[dw]))|madcss?wd|perm))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 454026305844151895,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vph(addu?(b[dqw]|w[dq]|dq)|sub(bw|dq|wd)))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 454026305844151960,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vfrcz[ps][ds]|vpermil2p[ds])\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 454026305844150497,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(femms)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 454026305845395456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(p(avgusb|(f2i|i2f)[dw]|mulhrw|swapd)|pf((p?n)?acc|add|max|min|mul|rcp(it[12])?|rsqit1|rsqrt|subr?))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 454026305845397079,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(pfcmp(eq|ge|gt))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 454026305845395769,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }