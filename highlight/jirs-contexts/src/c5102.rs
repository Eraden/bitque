
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
        a: 844721282875392,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844721282875392,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 5100 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi)\n\\b(create(?:\\s+or\\s+replace)?)\\s+\n(aggregate|conversion|database|domain|function|group|(?:unique\\s+)?index|language|operator class|operator|procedure|rule|schema|sequence|table(?:space)?|trigger|type|user|view)\n\\b\\s*"),
      scope: vec![
        Scope {
            a: 46449281036779520,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787102711877,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787017056256,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5101 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:\\s*\\b(drop)\\s+(aggregate|conversion|database|domain|function|group|index|language|operator class|operator|procedure|rule|schema|sequence|table|tablespace|trigger|type|user|view))"),
      scope: vec![
        Scope {
            a: 46446343279149056,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787102711877,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787017056256,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:\\s*(drop)\\s+(table)\\s+(\\w+)(\\s+cascade)?\\b)"),
      scope: vec![
        Scope {
            a: 46446343279149056,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787102711877,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787062014021,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130630615109,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636787102777413,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:\\s*\\b(alter)\\s+(aggregate|conversion|database|domain|function|group|index|language|operator class|operator|procedure|rule|schema|sequence|table|tablespace|trigger|type|user|view)\\s+)"),
      scope: vec![
        Scope {
            a: 46449289626714112,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787102711877,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787062014021,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi)\n\n        # normal stuff, capture 1\n         \\b(bigint|bigserial|bit|boolean|box|bytea|cidr|circle|date|datetime|double\\sprecision|inet|int|integer|line|lseg|macaddr|money|ntext|oid|path|point|polygon|real|serial|smallint|sysdate|sysname|text)\\b\n\n        # numeric suffix, capture 2 + 3i\n        |\\b(bit\\svarying|character\\s(?:varying)?|tinyint|var\\schar|float|interval)\\((\\d+)\\)\n\n        # optional numeric suffix, capture 4 + 5i\n        |\\b(char|number|nvarchar|varbinary|varchar\\d?)\\b(?:\\((\\d+)\\))?\n\n        # special case, capture 6 + 7i + 8i\n        |\\b(numeric|decimal)\\b(?:\\((\\d+),(\\d+)\\))?\n\n        # special case, captures 9, 10i, 11\n        |\\b(times?)\\b(?:\\((\\d+)\\))?(\\swith(?:out)?\\stime\\szone\\b)?\n\n        # special case, captures 12, 13, 14i, 15\n        |\\b(timestamp)(?:(s|tz))?\\b(?:\\((\\d+)\\))?(\\s(with|without)\\stime\\szone\\b)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576467050496,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576467050496,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955089166893056,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 48414576467050496,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 59955089166893056,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 48414576467050496,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 59955089166893056,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 59955089166893056,
            b: 0,
        },
    ]),(9, vec![
        Scope {
            a: 48414576467050496,
            b: 0,
        },
    ]),(10, vec![
        Scope {
            a: 59955089166893056,
            b: 0,
        },
    ]),(11, vec![
        Scope {
            a: 48414576467050496,
            b: 0,
        },
    ]),(12, vec![
        Scope {
            a: 48414576467050496,
            b: 0,
        },
    ]),(13, vec![
        Scope {
            a: 48414576467050496,
            b: 0,
        },
    ]),(14, vec![
        Scope {
            a: 59955089166893056,
            b: 0,
        },
    ]),(15, vec![
        Scope {
            a: 48414576467050496,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:\\b((?:primary|foreign)\\s+key|references|on\\sdelete(\\s+cascade)?|on\\supdate(\\s+cascade)?|check|constraint|default)\\b)"),
      scope: vec![
        Scope {
            a: 48414439028097024,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b\\d+\\b"),
      scope: vec![
        Scope {
            a: 59955089166893056,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:\\b(true|false)\\b)"),
      scope: vec![
        Scope {
            a: 59955480008916992,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:\\b(select(\\s+(distinct|top))?|insert(\\s+(ignore\\s+)?into)?|update|delete|truncate|from|set|where|group\\s+by|with|case|when|then|else|end|union(\\s+all)?|using|order\\s+by|limit|(inner|cross)\\s+join|join|straight_join|(left|right)(\\s+outer)?\\s+join|natural(\\s+(left|right)(\\s+outer)?)?\\s+join)\\b)"),
      scope: vec![
        Scope {
            a: 52636787102908485,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:\\b(?:(is)\\s+)?(?:(not)\\s+)?(null)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628114800709,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628114800709,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955110641729536,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:\\b(and|or|like|having|exists|between|in)\\b)"),
      scope: vec![
        Scope {
            a: 52636628114800709,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:\\bvalues\\b)"),
      scope: vec![
        Scope {
            a: 52636787102909796,
            b: 19421773393035264,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:\\b(begin(\\s+work)?|start\\s+transaction|commit(\\s+work)?|rollback(\\s+work)?)\\b)"),
      scope: vec![
        Scope {
            a: 52636787103039557,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:\\b(grant(\\swith\\sgrant\\soption)?|revoke)\\b)"),
      scope: vec![
        Scope {
            a: 52636787103105093,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:\\s*\\b(comment\\s+on\\s+(table|column|aggregate|constraint|database|domain|function|index|operator|rule|schema|sequence|trigger|type|view))\\s+.*?\\s+(is)\\s+)"),
      scope: vec![
        Scope {
            a: 52636787103170629,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\bas\\b"),
      scope: vec![
        Scope {
            a: 52636628111131073,
            b: 19421773393035264,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(asc|desc)\\b"),
      scope: vec![
        Scope {
            a: 52636787103236165,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*"),
      scope: vec![
        Scope {
            a: 49259061612970053,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<=>|[!<>]?=|<>|<|>"),
      scope: vec![
        Scope {
            a: 52636628119257157,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("-|\\+|/"),
      scope: vec![
        Scope {
            a: 52636628155629637,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\|\\|"),
      scope: vec![
        Scope {
            a: 52636628189577285,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(CURRENT_(DATE|TIME(STAMP)?|USER)|(SESSION|SYSTEM)_USER)\\b"),
      scope: vec![
        Scope {
            a: 61925255176454213,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(AVG|COUNT|MIN|MAX|SUM)(?=\\s*\\()"),
      scope: vec![
        Scope {
            a: 61925255176519749,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(CONCATENATE|CONVERT|LOWER|SUBSTRING|TRANSLATE|TRIM|UPPER)\\b"),
      scope: vec![
        Scope {
            a: 61925255098466373,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(\\w+?)\\.(\\w+)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955136498040901,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955136498106437,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5106 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5103 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\()(\\))"),
      scope: vec![
        Scope {
            a: 46444328939487232,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288522009084086,
            b: 19421773393035264,
        },
    ]),(2, vec![
        Scope {
            a: 47288522009084075,
            b: 19421773393035264,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }