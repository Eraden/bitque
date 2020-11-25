
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
        a: 844575253987328,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844575253987328,
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
      regex: Regex::new("(`)[a-zA-Z_\']*?(`)"),
      scope: vec![
        Scope {
            a: 52636628110344966,
            b: 9851624184872960,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324873763,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324873763,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\(\\)"),
      scope: vec![
        Scope {
            a: 59955110657982499,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\[\\]"),
      scope: vec![
        Scope {
            a: 59955110687997987,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(module)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787014828032,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2157 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(class)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787014828032,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2158 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(instance)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787014828032,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2159 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(import)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787014828032,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2160 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(deriving)\\s*\\("),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787014828032,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2161 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(deriving|where|data|type|case|of|let|in|newtype|default)\\b"),
      scope: vec![
        Scope {
            a: 52636787014828032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\binfix[lr]?\\b"),
      scope: vec![
        Scope {
            a: 52636628101038080,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(do|if|then|else)\\b"),
      scope: vec![
        Scope {
            a: 52636636690972672,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b\\d+(?:(\\.)\\d+(?:[eE][-+]?\\d+)?|(?:[eE][-+]?\\d+))\\b"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 9851624184872960,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397923,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[oO])[0-7]+\\b"),
      scope: vec![
        Scope {
            a: 59955089176461666,
            b: 9851624184872960,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 9851624184872960,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[xX])\\h+\\b"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 9851624184872960,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 9851624184872960,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b\\d+\\b"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 9851624184872960,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(#)\\s*\\w+"),
      scope: vec![
        Scope {
            a: 46444466374770688,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629327757325,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2176 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\""),
      scope: vec![
        Scope {
            a: 47288629323956406,
            b: 9851624184872960,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2162 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\[(?:|e|d|t|p)\\|"),
      scope: vec![
        Scope {
            a: 52636787062931491,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2163 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\$\\("),
      scope: vec![
        Scope {
            a: 52636787062997027,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\[[a-zA-Z0-9_\']*\\|"),
      scope: vec![
        Scope {
            a: 52636787062931491,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2164 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(\')\n(?:\n  [\\ -\\[\\]-~]                             # Basic Char\n  | (\\\\(?:NUL|SOH|STX|ETX|EOT|ENQ|ACK|BEL|BS|HT|LF|VT|FF|CR|SO|SI|DLE\n    |DC1|DC2|DC3|DC4|NAK|SYN|ETB|CAN|EM|SUB|ESC|FS|GS|RS\n    |US|SP|DEL|[abfnrtv\\\\\\\"\'\\&]))       # Escapes\n  | (\\\\o[0-7]+)                             # Octal Escapes\n  | (\\\\x[0-9A-Fa-f]+)                       # Hexadecimal Escapes\n  | (\\^[A-Z@\\[\\]\\\\\\^_])                     # Control Chars\n)\n(\')"),
      scope: vec![
        Scope {
            a: 55451420831973411,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956406,
            b: 9851624184872960,
        },
    ]),(2, vec![
        Scope {
            a: 59955200847314979,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955200847315298,
            b: 9851624184872960,
        },
    ]),(4, vec![
        Scope {
            a: 59955200847315160,
            b: 9851624184872960,
        },
    ]),(5, vec![
        Scope {
            a: 59955200847315134,
            b: 9851624184872960,
        },
    ]),(6, vec![
        Scope {
            a: 47288629323956395,
            b: 9851624184872960,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(\\s*)([a-z_][a-zA-Z0-9_\']*|\\([|!%$+\\-.,=</>]+\\))\\s*(::|∷)"),
      scope: vec![],
      captures: Some(vec![(2, vec![
        Scope {
            a: 59392130630615075,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636787038748707,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2165 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[A-Z]\\w*\\b"),
      scope: vec![
        Scope {
            a: 59955136409305088,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2171 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2172 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[|!%$?~+:\\-.=</>\\\\]+"),
      scope: vec![
        Scope {
            a: 52636628101038080,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(","),
      scope: vec![
        Scope {
            a: 47288620757155875,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }