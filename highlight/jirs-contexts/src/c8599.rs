
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
        a: 844987570847744,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844987570847744,
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
      regex: Regex::new("<#"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323038943,
            b: 51229008402055168,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8576 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[2-6]>&1|>>|>|<<|<|>|>\\||[1-6]>|[1-6]>>"),
      scope: vec![
        Scope {
            a: 52636628122140803,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8590 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8592 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8604 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8597 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8595 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8589 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8586 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8602 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8596 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8593 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8601 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8594 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\')\'"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 36873221949095936,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8577 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\@\"(?=(?m:$))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8578 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\@\'(?=(?m:$))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8579 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8600 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@)(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787024396470,
            b: 36873221949095936,
        },
    ]),(2, vec![
        Scope {
            a: 47288521944400054,
            b: 36873221949095936,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8580 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$)(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514563,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288521944400054,
            b: 36873221949095936,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8581 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\b(([A-Za-z0-9\\-_\\.]+)\\.(?i:exe|com|cmd|bat))\\b)"),
      scope: vec![
        Scope {
            a: 61925255094140928,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\w|-|\\.)((?i:begin|break|catch|continue|data|default|define|do|dynamicparam|else|elseif|end|exit|finally|for|from|if|in|inlinescript|parallel|param|process|return|sequence|switch|throw|trap|try|until|var|while)|%|\\?)(?!\\w)"),
      scope: vec![
        Scope {
            a: 52636636697264128,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\w|-|[^\\)]\\.)((?i:(foreach|where)(?!-object))|%|\\?)(?!\\w)"),
      scope: vec![
        Scope {
            a: 52636636697264128,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\w)(--%)(?!\\w)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636697264128,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8582 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\w)((?i:hidden|static))(?!\\w)"),
      scope: vec![
        Scope {
            a: 48414439032160256,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\w|-)((?i:class)|%|\\?)(?:\\s)+((?:\\p{L}|\\d|_|-|)+)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576471113728,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615040,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\w)-(?i:is(?:not)?|as)\\b"),
      scope: vec![
        Scope {
            a: 52636628119257219,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\w)-(?i:[ic]?(?:eq|ne|[gl][te]|(?:not)?(?:like|match|contains|in)|replace))(?!\\p{L})"),
      scope: vec![
        Scope {
            a: 52636628119257219,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\w)-(?i:join|split)(?!\\p{L})|!"),
      scope: vec![
        Scope {
            a: 52636628257931395,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\w)-(?i:and|or|not|xor)(?!\\p{L})|!"),
      scope: vec![
        Scope {
            a: 52636628114800771,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\w)-(?i:band|bor|bnot|bxor|shl|shr)(?!\\p{L})"),
      scope: vec![
        Scope {
            a: 52636628135903363,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\w)-(?i:f)(?!\\p{L})"),
      scope: vec![
        Scope {
            a: 52636628258783363,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[+%*/-]?=|[+/*%-]"),
      scope: vec![
        Scope {
            a: 52636628111130755,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\|{2}|&{2}|;"),
      scope: vec![
        Scope {
            a: 47288689454284931,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("&|(?<!\\w)\\.(?= )|`|,|\\|"),
      scope: vec![
        Scope {
            a: 52636628113490051,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\s|^)\\.\\.(?=\\-?\\d|\\(|\\$)"),
      scope: vec![
        Scope {
            a: 52636628116635779,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }