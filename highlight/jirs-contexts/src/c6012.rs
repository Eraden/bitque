
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 844790007988224,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844790007988224,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("@.*(?m:$)"),
      scope: vec![
        Scope {
            a: 51510711012032512,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("//.*(?m:$)"),
      scope: vec![
        Scope {
            a: 51510711012032512,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(";.*(?m:$)"),
      scope: vec![
        Scope {
            a: 51510711012032512,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*\\#\\s*if\\s+0\\b"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6001 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("/\\*"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6002 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n^\\s*\\#\\s*(define)\\s+             # define\n((?<id>[a-zA-Z_][a-zA-Z0-9_]*))  # macro name\n(?:                              # and optionally:\n    (\\()                         # an open parenthesis\n        (\n            \\s* \\g<id> \\s*       # first argument\n            ((,) \\s* \\g<id> \\s*)*  # additional arguments\n            (?:\\.\\.\\.)?          # varargs ellipsis?\n        )\n    (\\))                         # a close parenthesis\n)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636717449680,
            b: 3659174697238528,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615295,
            b: 3659174697238528,
        },
    ]),(4, vec![
        Scope {
            a: 47288629327560717,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 49258876855320589,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 47288620737626125,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6003 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*#\\s*(error|warning)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636717449485,
            b: 3659174697238528,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6004 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*#\\s*(include|import)\\b\\s+"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636717449670,
            b: 3659174697238528,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6005 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?i)([xw][0-9]|[xw]1[0-9]||[xw]2[0-9]|[wx]30|wzr|xzr|wsp|fpsr|fpcr|[rcp]1[0-5]|[rcp][0-9]|a[1-4]|v[1-8]|sl|sb|fp|ip|sp|lr|(c|s)psr(_c)?|pc|[sd]3[0-1]|[sd][12][0-9]|[sd][0-9]|fpsid|fpscr|fpexc|q3[0-1]|q2[0-9]|q1[0-9]|q[0-9]|APSR_nzcv|sy)!?(?-i))?\\b"),
      scope: vec![
        Scope {
            a: 48414662466076672,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\.(?i)(globl|global|macro|endm|purgem|if|elseif|else|endif|section|text|arm|align|balign|irp|rept|endr|req|unreq|error|short|func|endfunc|hidden|type|fpu|arch|code|altmacro|object_arch|word|int|string)(?-i)\\b"),
      scope: vec![
        Scope {
            a: 52636636728262656,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("armv(2a?|3m?|4t?|5t?e?6(j|t2|zk?|-m)?|7v?e?(-(a|r|m))?|8-a(\\+crc)?)"),
      scope: vec![
        Scope {
            a: 52636636792946774,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*#\\s*(define|defined|elif|else|if|ifdef|ifndef|line|pragma|undef|endif)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636717449229,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6008 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\\b((?i)\n(\n  (bf(c|i)|(u|s)bfx|(u|s)xta?(h|b)?) |\n  (v(add|cvt|sub|mov|trn|cmp|div|qdmulh|mrs|mul|ld1|qadd|qshrun|st[1234]|addw|mull|mlal|rshrn|swp|qmovun)|qmovun)(\\.([isup]?8|[isupf]?16|[isuf]?32|[isu]?64))* |\n  (and|m(rs|sr)|eor|sub|rsb|add|adc|sbc|rsc|tst|teq|cmp|cmn|orr|mov|bic|mvn |\n    (neg) |\n    (lsr|lsl|ror|asr) # shift ops either pseudo ops or actual shifts\n  )s? |\n  (mul|mla|mull|smlabb) |\n  (mov(w|t)) |\n  rev(8|16)? |\n  (pld|adr|adrl|vswp)\n)\n(ne|eq|cs|hs|cc|lo|mi|pl|vs|vc|hi|ls|lt|le|gt|ge|al)?(?-i))?\\b"),
      scope: vec![
        Scope {
            a: 61925255189889336,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\\b((?i)(\n  swi|svc|wfi|\n  dmb | clrex | dsb | isb |\n  v(ldr|str|push|pop) |\n  (push|pop) |\n  (st|ld)(\n      p |\n    r(ex|s?(h|b)|d)? |\n    m(\n      (f|e)(d|a) |\n      (d|i)(b|a)\n    )?\n  ) |\n  b(l|x|lx|lr|r)? |\n  (i|e)?ret|\n  b\\.(eq|ne|hs|cs|lo|cc|mi|pl|vs|vc|hi|ls|ge|lt|gt|le|al|nv)+ |\n  (c|t)?bn?z|\n)+(ne|eq|cs|hs|cc|lo|mi|pl|vs|vc|hi|ls|lt|le|gt|ge|al)?(?-i))\\b"),
      scope: vec![
        Scope {
            a: 61925255189890617,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((?i)(def(b|w|s)|equ|(include|get)(\\s+([a-zA-Z_]+[0-9a-zA-Z_]*|[0-9]+[a-zA-Z_]+[0-9a-zA-Z_]*?)\\.s)?)?(?-i))\\b"),
      scope: vec![
        Scope {
            a: 46444466374771142,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((?i)(align)(?-i))?\\b"),
      scope: vec![
        Scope {
            a: 48414576463382069,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s+\\\".+\\\""),
      scope: vec![
        Scope {
            a: 55450570411999232,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((?i)nop(ne|eq|cs|hs|cc|lo|mi|pl|vs|vc|hi|ls|lt|le|gt|ge|al)?(?-i))?\\b"),
      scope: vec![
        Scope {
            a: 51516766915919872,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s\\["),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6009 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\b|\\s+)\\=\\b"),
      scope: vec![
        Scope {
            a: 52636636736192512,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\b|\\s+)(\\#)?-?(0x|&)[0-9a-fA-F_]+\\b"),
      scope: vec![
        Scope {
            a: 59955089201168384,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\b|\\s+)\\#-?[0-9a-zA-Z_]+\\b"),
      scope: vec![
        Scope {
            a: 59955089177444352,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\b|\\s+)[0-9]+\\b"),
      scope: vec![
        Scope {
            a: 59955089266900992,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b([a-zA-Z_]+[0-9a-zA-Z_]*|[0-9]+[a-zA-Z_]+[0-9a-zA-Z_]*)\\b"),
      scope: vec![
        Scope {
            a: 46444131366666326,
            b: 99360666778861568,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }