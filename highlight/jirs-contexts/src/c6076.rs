
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
      regex: Regex::new("(?i)\\b(vblendm(pd|ps)|vpblendm[bdqw])\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 459937091267067904,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vbroadcast[fi](32x[248]|64x[24])|v(extract|insert)[fi](32x[48]|64x[24])|vshuf[fi](32x4|64x2)|vpbroadcastm(b2q|w2d))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 459937134216740864,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(v(compress|expand)p[ds]|vp(compress|expand|conflict)[dq])\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 459937138511708160,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vcvt(t?p[sd]2(udq|u?qq)|(udq|u?qq)2p[ds]|t?s[ds]2usi|usi2s[ds]))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 459935592323481600,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(v(fixupimm|fpclass|get(exp|mant)|range|(rcp|rsqrt)(14|28)|reduce|rndscale|scalef)([ps][ds]))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 459933839976824832,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(v(exp2p[ds]|(scatter|(gather|scatter)pf[01])[dq]p[ds]))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 459933839976824832,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vmovdq(a(32|64)|u(8|16|32|64)))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 459937048317394944,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vp(andn?|x?or)[dq])\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 459931164212199424,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vpcmpu?[dqw])\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 459937142806675456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vp(absq|(lzcnt|ternlog)[dq]|madd52[lh]uq|(max|min)[su]q|mullq))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 459937147101642752,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vpmov(m2[bdqw]|[bdqw]2m|(u?s)?([qd][bw]|qd|wb)))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 459937151396610048,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vp(ro[rl]v?[dq]|scatter[dq][dq]))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 459937155691577344,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vptestn?m[bdqw])\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 459937159986544640,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vperm([bdw]|[it]2([bdwq]|p[ds])))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 459937164281511936,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(valign[dq]|vdbpsadbw|vpmultishiftqb|vpsrav[dqw])\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 459931078312853504,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }