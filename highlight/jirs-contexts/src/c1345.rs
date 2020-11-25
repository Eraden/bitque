
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
        index: 1314,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 1182 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("!"),
      scope: vec![
        Scope {
            a: 52636628114800656,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1345 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("~"),
      scope: vec![
        Scope {
            a: 52636628135903248,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1345 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\+\\+|--|\\+|-)"),
      scope: vec![
        Scope {
            a: 52636628119191568,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1345 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\*|&)"),
      scope: vec![
        Scope {
            a: 52636628135903248,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1345 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bcast\\b"),
      scope: vec![
        Scope {
            a: 52636628119322640,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1153 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bdelete\\b"),
      scope: vec![
        Scope {
            a: 52636628119322640,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1345 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bnew\\b"),
      scope: vec![
        Scope {
            a: 52636628119322640,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1154 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bimport\\b"),
      scope: vec![
        Scope {
            a: 52636636717449232,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1156 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bmixin\\b"),
      scope: vec![
        Scope {
            a: 52636636689727488,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1157 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bassert\\b"),
      scope: vec![
        Scope {
            a: 52636787048972304,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1346 }),
        ContextReference::Direct(ContextId { index: 1178 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bis\\b"),
      scope: vec![
        Scope {
            a: 52636787013582848,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1158 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\btypeof\\b"),
      scope: vec![
        Scope {
            a: 52636787013582848,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1159 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b__traits\\b"),
      scope: vec![
        Scope {
            a: 52636787013582848,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1160 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(function|delegate)\\b"),
      scope: vec![
        Scope {
            a: 46444131367518208,
            b: 0,
        },
        Scope {
            a: 52636787013582848,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1163 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\()\\s*(\\))"),
      scope: vec![
        Scope {
            a: 46444131382984720,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521948004534,
            b: 4503599627370496,
        },
    ]),(2, vec![
        Scope {
            a: 47288521948004523,
            b: 4503599627370496,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1346 }),
        ContextReference::Direct(ContextId { index: 1355 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 47288521944400054,
            b: 4503599627370496,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1155 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?={)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1346 }),
        ContextReference::Direct(ContextId { index: 1193 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\["),
      scope: vec![
        Scope {
            a: 47288521961570486,
            b: 4503599627370496,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1350 }),
        ContextReference::Direct(ContextId { index: 1347 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\b(const|immutable|inout|shared)\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1346 }),
        ContextReference::Direct(ContextId { index: 1335 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(\\b|\\.)\\d)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1346 }),
        ContextReference::Direct(ContextId { index: 1304 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\')"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1346 }),
        ContextReference::Direct(ContextId { index: 1199 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=`|[rxq]?\"|q{)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1346 }),
        ContextReference::Direct(ContextId { index: 1320 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(null|true|false|__FILE__|__FILE_FULL_PATH__|__MODULE__|__LINE__|__FUNCTION__|__PRETTY_FUNCTION__|__DATE__|__EOF__|__TIME__|__TIMESTAMP__|__VENDOR__|__VERSION__|__ctfe)\\b"),
      scope: vec![
        Scope {
            a: 59955110638256128,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1346 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(this|super)\\b"),
      scope: vec![
        Scope {
            a: 49259061523251200,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1346 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\$"),
      scope: vec![
        Scope {
            a: 49259061523251200,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1346 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(bool|byte|cdouble|cent|cfloat|char|creal|dchar|double|float|idouble|ifloat|int|ireal|long|real|short|ubyte|ucent|uint|ulong|ushort|void|wchar|string|dstring|wstring)\\b"),
      scope: vec![
        Scope {
            a: 48414576463577088,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1346 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b([\\p{L}_][\\p{L}0-9_]*)\\s*(=>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49258876839657472,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576474128400,
            b: 0,
        },
        Scope {
            a: 52638212953276789,
            b: 4503599627370496,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1285 }),
        ContextReference::Direct(ContextId { index: 1345 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\."),
      scope: vec![
        Scope {
            a: 46445252353982464,
            b: 0,
        },
        Scope {
            a: 47288788226932752,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1346 }),
        ContextReference::Direct(ContextId { index: 1356 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b([\\p{L}_][\\p{L}0-9_]*)\\s*(:)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087293054976,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620757876985,
            b: 4503599627370496,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1345 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\b[\\p{L}_][\\p{L}0-9_]*\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1346 }),
        ContextReference::Direct(ContextId { index: 1356 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1303 })),
]
} }