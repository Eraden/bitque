
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
        a: 844867311763456,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844867311763456,
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
            b: 28991922601197568,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324873831,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324873831,
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
            a: 59955110657982567,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\b((effect|port)\\s+)?(module)\\s+"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787019284480,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636787019284480,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6945 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\b(import)\\s+((open)\\s+)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787019284480,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 50102545854496768,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6947 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\[)(glsl)(\\|)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787019284480,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 61925255135625319,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636787019284480,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6948 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("type( alias)? (\\w+)"),
      scope: vec![
        Scope {
            a: 52636787019284480,
            b: 0,
        },
    ],
      captures: Some(vec![(2, vec![
        Scope {
            a: 59392130632450151,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(case|of|let|in|as)\\s+"),
      scope: vec![
        Scope {
            a: 52636787019284480,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(if|then|else)\\s+"),
      scope: vec![
        Scope {
            a: 52636636695429120,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b([0-9]+\\.[0-9]+([eE][+-]?[0-9]+)?|[0-9]+[eE][+-]?[0-9]+)\\b"),
      scope: vec![
        Scope {
            a: 59955089176592487,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b([0-9]+)\\b"),
      scope: vec![
        Scope {
            a: 59955089169121280,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\"\"\""),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 28991922601197568,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6949 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\""),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 28991922601197568,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6950 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(\')\n(?:\n  [\\ -\\[\\]-~]               # Basic Char\n  | (\\\\(?:NUL|SOH|STX|ETX|EOT|ENQ|ACK|BEL|BS|HT|LF|VT|FF|CR|SO|SI|DLE\n    |DC1|DC2|DC3|DC4|NAK|SYN|ETB|CAN|EM|SUB|ESC|FS|GS|RS\n    |US|SP|DEL|[abfnrtv\\\\\\\"\'\\&]))   # Escapes\n  | (\\^[A-Z@\\[\\]\\\\\\^_])           # Control Chars\n)\n(\')"),
      scope: vec![
        Scope {
            a: 55451420831973479,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956406,
            b: 28991922601197568,
        },
    ]),(2, vec![
        Scope {
            a: 59955200847315047,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323956395,
            b: 28991922601197568,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(port\\s+)?([a-z_][a-zA-Z0-9_\']*|\\([|!%$+\\-.,=</>]+\\))\\s*((:)([:]+)?)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787156516967,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615143,
            b: 0,
        },
        Scope {
            a: 618687526577045504,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636787034947687,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 50102545854496768,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6951 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bport\\s+"),
      scope: vec![
        Scope {
            a: 52636787156516967,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[A-Z]\\w*\\b"),
      scope: vec![
        Scope {
            a: 59955136413761536,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6956 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^[a-z][A-Za-z0-9_\']*\\s+"),
      scope: vec![
        Scope {
            a: 59392130630615143,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6957 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[|!%$?~+:\\-.=</>&\\\\*^]+"),
      scope: vec![
        Scope {
            a: 52636628105494528,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([\\[\\]\\{\\},])"),
      scope: vec![
        Scope {
            a: 59955110738722919,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255187071079,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([\\(\\)])"),
      scope: vec![
        Scope {
            a: 52636787068239975,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }