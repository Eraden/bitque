
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
      regex: Regex::new("(?i)\\b(?:mov(?:[sz]x)?|cmov(?:n?[abceglopsz]|n?[abgl]e|p[eo]))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 463033329001234432,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(xchg|bswap|xadd|cmpxchg(8b)?)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 463033329003659264,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b((push|pop)(ad?)?|cwde?|cdq|cbw)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 463033328910532608,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(adcx?|adox|add|sub|sbb|i?mul|i?div|inc|dec|neg|cmp)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 463033410500165632,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(daa|das|aaa|aas|aam|aad)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 463033414795132928,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(and|x?or|not)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 463027388956016640,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(s[ah][rl]|sh[rl]d|r[co][rl])\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 463033419090100224,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(set(n?[abceglopsz]|n?[abgl]e|p[eo]))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 463033423401254912,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(bt[crs]?|bs[fr]|test|crc32|popcnt)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 463033423399813120,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(jmp|jn?[abceglopsz]|jn?[abgl]e|jp[eo]|j[er]?cxz)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 463033427788300288,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(loop(n?[ez])?|call|ret|iret[dq]?|into?|bound|enter|leave)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 463033427694780416,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b((mov|cmp|sca|lod|sto)(s[bdw]?)|rep(n?[ez])?)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 463033436269969408,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b((in|out)(s[bdw]?)?)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 463033440564936704,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b((st|cl)[cdi]|cmc|[ls]ahf|(push|pop)f[dq]?)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 463033444859904000,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(l[defgs]s)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 463033449154871296,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(lea|nop|ud2|xlatb?|cpuid|movbe)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 463031980276056064,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(rdrand|rdseed)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 463033453449838592,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(andn|bextr|bls(i|r|msk)|bzhi|pdep|pext|[lt]zcnt|(mul|ror|sar|shl|shr)x)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 463033457744805888,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }