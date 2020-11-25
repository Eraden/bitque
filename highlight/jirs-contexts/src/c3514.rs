
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
        a: 844648268431360,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844648268431360,
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
      regex: Regex::new("\\b(let)\\s+(module)\\s+([A-Z][a-zA-Z0-9\'_]*)\\s*(=)"),
      scope: vec![
        Scope {
            a: 46446648270782464,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787081936948,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787082002484,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925461293989940,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288620790513716,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(let|and)\\s+(open)\\s+[A-Z][a-zA-Z0-9\'_]*(\\.[A-Z][a-zA-Z0-9\'_]*)*\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787015942144,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636636717449268,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(let|and)\\s+(?!\\(\\*)((rec\\s+)([a-z_][a-zA-Z0-9_\']*)\\b|([a-z_][a-zA-Z0-9_\']*|\\([^)]+\\))(?=\\s)((?=\\s*=\\s*(?=fun(?:ction)\\b))|(?!\\s*=)))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787079839796,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636787079839796,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130630615092,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 59392130630615092,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3468 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(let|and)\\s+([a-z][a-zA-Z0-9\'_]*)\\s*(=)\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787015942144,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087305965620,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636628111130676,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\(|\\s)(?=fun\\s)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322645876,
            b: 14636698788954112,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3469 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(?=type\\s)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3483 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(with|function)(?=(\\s*(?m:$)|.*->))\\b|((?<!\\S)(\\|)(?=(\\w|\\s).*->))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636757033012,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636636757033012,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3485 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^[ \\t]*(class\\s+type\\s+)((\\[\\s*(\'[A-Za-z][a-zA-Z0-9_\']*(?:\\s*,\\s*\'[A-Za-z][a-zA-Z0-9_\']*)*)\\s*\\]\\s+)?[a-z_][a-zA-Z0-9\'_]*)"),
      scope: vec![
        Scope {
            a: 46444243103842356,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787082068020,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632451110,
            b: 14636698788954112,
        },
    ]),(4, vec![
        Scope {
            a: 48414576465936384,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^[ \\t]*(class)(?:\\s+(?!(?:virtual)\\s+))((\\[\\s*(\'[A-Za-z][a-zA-Z0-9_\']*(?:\\s*,\\s*\'[A-Za-z][a-zA-Z0-9_\']*)*)\\s*\\]\\s+)?[a-z_][a-zA-Z0-9\'_]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787082199092,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632450251,
            b: 14636698788954112,
        },
    ]),(4, vec![
        Scope {
            a: 48414576465936384,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3486 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^[ \\t]*(class\\s+virtual\\s+)((\\[\\s*(\'[A-Za-z][a-zA-Z0-9_\']*(?:\\s*,\\s*\'[A-Za-z][a-zA-Z0-9_\']*)*)\\s*\\]\\s+)?[a-z_][a-zA-Z0-9\'_]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787082199092,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632450251,
            b: 14636698788954112,
        },
    ]),(4, vec![
        Scope {
            a: 48414576465936384,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3487 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^[ \\t]*(class\\s+type\\s+virtual)((\\[\\s*(\'[A-Za-z][a-zA-Z0-9_\']*(?:\\s*,\\s*\'[A-Za-z][a-zA-Z0-9_\']*)*)\\s*\\]\\s+)?[a-z_][a-zA-Z0-9\'_]*)"),
      scope: vec![
        Scope {
            a: 46444243104105489,
            b: 14636698788954112,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787082068020,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632451110,
            b: 14636698788954112,
        },
    ]),(4, vec![
        Scope {
            a: 48414576465936384,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629349777460,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3488 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(object)\\s*(?:(\\()(_?[a-z]+)(\\)))?\\s*(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787082264628,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629380841524,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632451113,
            b: 14636698788954112,
        },
    ]),(4, vec![
        Scope {
            a: 47288629380841524,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3470 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=\\w|\\)|\')(#)[a-z_][a-zA-Z0-9\'_]*"),
      scope: vec![
        Scope {
            a: 46445286716080128,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620750340148,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^[ \\t]*(module)\\s+([A-Z_][a-zA-Z0-9\'_]*)(?:\\s*(:)\\s*([A-Z][a-zA-Z0-9\'_]*)?)?"),
      scope: vec![
        Scope {
            a: 46446648220712960,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787082002484,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632450811,
            b: 14636698788954112,
        },
    ]),(3, vec![
        Scope {
            a: 47288620790579252,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130632451114,
            b: 14636698788954112,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^[ \\t]*(module\\s+type\\s+)([A-Z][a-zA-Z0-9\'_]*)"),
      scope: vec![
        Scope {
            a: 46446648230740020,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787082461236,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632451114,
            b: 14636698788954112,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(sig)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787062539271,
            b: 14636698788954112,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3473 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(struct)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787062538971,
            b: 14636698788954112,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3474 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3517 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(open)\\s+([A-Z][a-zA-Z0-9\'_]*)(?=(\\.[A-Z][a-zA-Z0-9_]*)*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787015942144,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 61925461293989940,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3475 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(exception)\\s+([A-Z][a-zA-Z0-9\'_]*)\\b"),
      scope: vec![
        Scope {
            a: 46444517916934144,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787015942144,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632450315,
            b: 14636698788954112,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(\\[<)(?![^\\[]+?[^>]]))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3476 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3519 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3511 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3510 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3513 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3509 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\()(?=(~[a-z][a-zA-Z0-9_]*:|(\"(\\\\\"|[^\"])*\")|[^\\(\\)~\"])+(?<!:)(:>|:(?![:=])))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288522004103220,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3477 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^[ \\t]*#[a-zA-Z]+"),
      scope: vec![
        Scope {
            a: 52636787052118068,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^[ \\t]*#[0-9]*"),
      scope: vec![
        Scope {
            a: 52636787052118587,
            b: 14636698788954112,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3518 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(mutable|ref)\\b"),
      scope: vec![
        Scope {
            a: 52636787023806637,
            b: 14636698788954112,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("`[A-Za-z][a-zA-Z0-9\'_]*\\b"),
      scope: vec![
        Scope {
            a: 59392130632451116,
            b: 300896973441990656,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[A-Z][a-zA-Z0-9\'_]*\\b"),
      scope: vec![
        Scope {
            a: 59392130632451116,
            b: 14636698788954112,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("!=|:=|>|<"),
      scope: vec![
        Scope {
            a: 52636628137082932,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[*+/-]\\."),
      scope: vec![
        Scope {
            a: 52636628149470254,
            b: 14636698788954112,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("~-\\."),
      scope: vec![
        Scope {
            a: 52636628132103214,
            b: 14636698788954112,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("::"),
      scope: vec![
        Scope {
            a: 47288629329002860,
            b: 14636698788954112,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(";;"),
      scope: vec![
        Scope {
            a: 47288689463132212,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(";"),
      scope: vec![
        Scope {
            a: 47288620724518912,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("->"),
      scope: vec![
        Scope {
            a: 47288620791300148,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[=<>@^&+\\-*/$%|][|!$%&*+./:<=>?@^~-]*"),
      scope: vec![
        Scope {
            a: 52636628149469236,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bnot\\b|!|[!\\?~][!$%&*+./:<=>?@^~-]+"),
      scope: vec![
        Scope {
            a: 52636628132102196,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("~[a-z][a-z0-9\'_]*(:)?"),
      scope: vec![
        Scope {
            a: 59392130632122721,
            b: 14636698788954112,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620791365684,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(begin)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636756574260,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3478 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(for)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636756639796,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3479 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(while)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636756705332,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3481 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3482 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(and|land|lor|lsl|lsr|lxor|mod|or)\\b"),
      scope: vec![
        Scope {
            a: 52636628102152192,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(downto|if|else|match|then|to|when|with|try)\\b"),
      scope: vec![
        Scope {
            a: 52636636692086784,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(as|assert|class|constraint|exception|functor|in|include|inherit|initializer|lazy|let|mod|module|mutable|new|object|open|private|rec|sig|struct|type|virtual)\\b"),
      scope: vec![
        Scope {
            a: 52636787015942144,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3516 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(’|‘|“|”)"),
      scope: vec![
        Scope {
            a: 50103314723962932,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }