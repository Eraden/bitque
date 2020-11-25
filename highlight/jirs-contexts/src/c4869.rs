
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
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b([\\p{Lu}]\\w*)(?=(\\s*,\\s*[\\p{Lu}]\\w*)*\\s*=(?![=\\>]))"),
      scope: vec![
        Scope {
            a: 46444285989617664,
            b: 0,
        },
        Scope {
            a: 59392130632974402,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b[?!]?(:)(?!:)"),
      scope: vec![
        Scope {
            a: 59955136445349954,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325004866,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4955 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(nil|true|false)\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 59955110641532928,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(__(FILE|LINE|ENCODING)__|self)\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 49259061526528000,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[xX])(?:\\h+(?:_\\h+)*)(r?i)(r)?\\b"),
      scope: vec![
        Scope {
            a: 59955089199268056,
            b: 18577348462903296,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 18577348462903296,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553282,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 50103314667667522,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[oO]?)(?:[0-7]+(?:_[0-7]+)*)(r?i)(r)?\\b"),
      scope: vec![
        Scope {
            a: 59955089199268194,
            b: 18577348462903296,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 18577348462903296,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553282,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 50103314667667522,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[bB])(?:[01]+(?:_[01]+)*)(r?i)(r)?\\b"),
      scope: vec![
        Scope {
            a: 59955089199268269,
            b: 18577348462903296,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 18577348462903296,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553282,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 50103314667667522,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?x:\n  (?:\n    # 0d1i, 0d1ri, 1i, 1ri | 1.1i, 1.1ri\n    (?: (0[dD])? (?:\\d+(?:_\\d+)*) | (?:\\d+(?:_\\d+)*) (\\.) (?:\\d+(?:_\\d+)*) ) (r?i)\n    # 1e1i, 1.1e1i\n    | (?:\\d+(?:_\\d+)*) (?: (\\.) (?:\\d+(?:_\\d+)*) )? (?:[Ee][-+]?(?:\\d+(?:_\\d+)*)) (r)? (i)\n  ) (r)?\n)\\b"),
      scope: vec![
        Scope {
            a: 59955089199268058,
            b: 18577348462903296,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 18577348462903296,
        },
    ]),(2, vec![
        Scope {
            a: 47288620735397954,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576476553282,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288620735397954,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 50103314667667522,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 48414576476553282,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 50103314667667522,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[xX])(?:\\h+(?:_\\h+)*)(r)\\b"),
      scope: vec![
        Scope {
            a: 59955089198350552,
            b: 18577348462903296,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 18577348462903296,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553282,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[oO]?)(?:[0-7]+(?:_[0-7]+)*)(r)\\b"),
      scope: vec![
        Scope {
            a: 59955089198350690,
            b: 18577348462903296,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 18577348462903296,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553282,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[bB])(?:[01]+(?:_[01]+)*)(r)\\b"),
      scope: vec![
        Scope {
            a: 59955089198350765,
            b: 18577348462903296,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 18577348462903296,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553282,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:(0[dD])?(?:\\d+(?:_\\d+)*)|(?:\\d+(?:_\\d+)*)(\\.)(?:\\d+(?:_\\d+)*))(r)\\b"),
      scope: vec![
        Scope {
            a: 59955089198350554,
            b: 18577348462903296,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 18577348462903296,
        },
    ]),(2, vec![
        Scope {
            a: 47288620735397954,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576476553282,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:\\d+(?:_\\d+)*)(?:(\\.)(?:\\d+(?:_\\d+)*)|(?:(\\.)(?:\\d+(?:_\\d+)*))?(?:[Ee][-+]?(?:\\d+(?:_\\d+)*))(r)?)\\b"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 18577348462903296,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397954,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620735397954,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 50103314667668005,
            b: 18577348462903296,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[xX])(?:\\h+(?:_\\h+)*)\\b"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 18577348462903296,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 18577348462903296,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[oO]?)(?:[0-7]+(?:_[0-7]+)*)\\b"),
      scope: vec![
        Scope {
            a: 59955089176461666,
            b: 18577348462903296,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 18577348462903296,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[bB])(?:[01]+(?:_[01]+)*)\\b"),
      scope: vec![
        Scope {
            a: 59955089176461741,
            b: 18577348462903296,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 18577348462903296,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[dD])?(?:\\d+(?:_\\d+)*)\\b"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 18577348462903296,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 18577348462903296,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(":\'"),
      scope: vec![
        Scope {
            a: 47288629325004866,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4791 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(":\""),
      scope: vec![
        Scope {
            a: 47288629325004866,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4792 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  (:)\n  (\n    \\b[\\p{L}_][\\p{L}\\p{N}_]*\\b(?:[?!]|=(?![>=]))?|\n    ===?|\n    >[>=]?|\n    <[<=]?|\n    <=>|\n    [%&`/\\|]|\n    \\*\\*?|\n    =?~|\n    [-+]@?|\n    \\[\\]=?|\n    @@?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\n  )\n)"),
      scope: vec![
        Scope {
            a: 59955136445349954,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325004866,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\?)\\\\u)(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955200835846144,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325004866,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 46445029018959872,
            b: 0,
        },
        Scope {
            a: 47288521962160310,
            b: 18577348462903296,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4793 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x: (\\?)(?:\n  # examples (meta control sequences):\n  # ?\\C-a    ?\\M-a    ?\\M-\\C-a    ?\\ca    ?\\M-ca\n  (?:\\\\(?:[MC]-|c)){1,2}[[:ascii:]] |\n  \\\\(?:\n    # examples (hex):\n    # ?\\x1     ?\\x61\n    x\\h{1,2}\\b |\n    # examples (octal):\n    # ?\\0      ?\\07     ?\\017\n    0[0-7]{0,2}\\b |\n    # examples (escaped):\n    # ?\\n      ?\\b      ?\\\\\n    .(?!\\w)\n  ) |\n  # examples (illegal):\n  # ?abc     ?\\xAG    ?\\\\n\n  # ?_a0\n  ([\\p{L}_\\\\]\\S+)\\b(?!\\s*:) |\n  # examples (normal):\n  # ?a       ?A       ?0\n  # ?*       ?\"       ?(\n  # ?.       ?#       ?\\\n  [\\p{L}\\p{N}_]\\b(?!\\s*:) | [^\\p{L}\\p{N}_\\s]\n) )"),
      scope: vec![
        Scope {
            a: 59955200835846144,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325004866,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 50103314669371458,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }