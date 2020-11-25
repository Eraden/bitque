
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
      regex: Regex::new("(?i)\\b(v((test|permil|maskmov)p[ds]|zero(all|upper)|(perm2|insert|extract|broadcast)f128|broadcasts[ds]))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451767337620602880,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vaes((dec|enc)(last)?|imc|keygenassist)|vpclmulqdq)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235443396608,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(v((cmp[ps]|u?comis)[ds]|pcmp([ei]str[im]|(eq|gt)[bdqw])))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235358593024,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(v(cvt(dq2pd|dq2ps|pd2ps|ps2pd|sd2ss|si2sd|si2ss|ss2sd|t?(pd2dq|ps2dq|sd2si|ss2si))))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235421704192,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vh((add|sub)p[ds])|vph((add|sub)([dw]|sw)|minposuw))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235443462144,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(v((andn?|x?or)p[ds]))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235354136576,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(v(mov(([ahl]|msk|nt|u)p[ds]|(hl|lh)ps|s([ds]|[hl]dup)|q)))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235443527680,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(v((add|div|mul|sub|max|min|round|sqrt)[ps][ds]|(addsub|dp)p[ds]|(rcp|rsqrt)[ps]s))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235443593216,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(v(pack[su]s(dw|wb)|punpck[hl](bw|dq|wd|qdq)|unpck[hl]p[ds]))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235443658752,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vp(shuf([bd]|[hl]w))|vshufp[ds])\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235443724288,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vp((abs|sign|(max|min)[su])[bdw]|(add|sub)([bdqw]|u?s[bw])|avg[bw]|extr[bdqw]|madd(wd|ubsw)|mul(hu?w|hrsw|l[dw]|u?dq)|sadbw))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235443790136,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vp(andn?|x?or))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235443790069,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vpblend(vb|w))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235443791438,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vpmov(mskb|[sz]x(b[dqw]|w[dq]|dq)))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235443791433,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vp(insr[bdqw]|sll(dq|[dqw])|srl(dq)))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235443920896,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vp(sra[dwq]|srl[dqw]))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235443986432,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vblendv?p[ds])\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235444051968,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vp(test|alignr))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235444117504,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vmov(d(dup|qa|qu)?))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235443922505,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(v((extract|insert)ps|lddqu|(ld|st)mxcsr|mpsadbw))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235352825856,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(v(maskmovdqu|movntdqa?))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 451774235444183040,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vcvt(ph2ps|ps2ph))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 455989462271262720,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vfn?m((add|sub)(132|213|231)[ps][ds])|vfm((addsub|subadd)(132|213|231)p[ds]))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 456270937247973376,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }