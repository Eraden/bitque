
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 4107 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b((?:require|include)(?:_once)?)\\b\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636717449670,
            b: 16325548649218048,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4003 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\byield\\b"),
      scope: vec![
        Scope {
            a: 52636636692480000,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4004 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  break | case | continue | declare | default | die | do | else |\n  elseif | enddeclare | endfor | endforeach | endif | endswitch | endwhile | exit |\n  for | foreach | if | return | switch | use | while\n)\\b"),
      scope: vec![
        Scope {
            a: 52636636692480000,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(catch)\\b\\s*\\(\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636706177426,
            b: 16325548649218048,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4005 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(catch|try|throw|finally)\\b"),
      scope: vec![
        Scope {
            a: 52636636706177082,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4099 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4106 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4132 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\\s*(?=\n    [\\p{L}\\p{N}_$\\\\]+(::)\n    (?:\n        (\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*\\(\n        |\n        ((\\$+)\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\n        |\n        (\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\n    )?\n)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4006 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4156 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4147 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(array)(\\()(\\))"),
      scope: vec![
        Scope {
            a: 46444148594376762,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255159676986,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288521948725430,
            b: 16325548649218048,
        },
    ]),(3, vec![
        Scope {
            a: 47288521948725419,
            b: 16325548649218048,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(array)(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255159676986,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288521948725430,
            b: 16325548649218048,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4008 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\s*\\(\\s*(array|real|double|float|int(eger)?|bool(ean)?|string|object|binary|unset)\\s*\\)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576466329600,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  array | bool | boolean | class | clone | double | float | function |\n  int | integer | interface | object | real | string\n)\\b"),
      scope: vec![
        Scope {
            a: 48414576466329600,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(parent|self)\\b"),
      scope: vec![
        Scope {
            a: 49259061526003712,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?xi:\n  abstract | const | extends | final | global | implements | private | protected |\n  public | static\n)\\b"),
      scope: vec![
        Scope {
            a: 48414439027376128,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4137 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(";"),
      scope: vec![
        Scope {
            a: 47288689463132218,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4120 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4136 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\+|-|\\*|/|%|&|\\||\\^|>>|<<|\\.|\\?\\?)="),
      scope: vec![
        Scope {
            a: 52636628111130982,
            b: 16325548649218048,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("=>"),
      scope: vec![
        Scope {
            a: 52636628118077498,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(\\=)(&))|(&(?=[\\p{L}$_]))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628111130682,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414439034978362,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414439034978362,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@)"),
      scope: vec![
        Scope {
            a: 52636628173652026,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\-\\-|\\+\\+)"),
      scope: vec![
        Scope {
            a: 52636628147372090,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\-|\\+|\\*|/|%)"),
      scope: vec![
        Scope {
            a: 52636628119191610,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("&&|\\|\\|"),
      scope: vec![
        Scope {
            a: 52636628114800698,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<<|>>|~|\\^|&|\\|"),
      scope: vec![
        Scope {
            a: 52636628135903290,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(===|!==|<=>|==|!=|<=|>=|<>|<|>)"),
      scope: vec![
        Scope {
            a: 52636628119257146,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)(!)|\\b(and|or|xor|as)\\b"),
      scope: vec![
        Scope {
            a: 52636628114800698,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4114 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\.\\.\\."),
      scope: vec![
        Scope {
            a: 52636628152877114,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\."),
      scope: vec![
        Scope {
            a: 52636628111654970,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("="),
      scope: vec![
        Scope {
            a: 52636628111130682,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(instanceof)\\s+(?=[\\p{L}\\\\$_])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628112179258,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4009 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4130 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)(goto)\\s+(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636716335162,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 61925461247787008,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4144 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\[)\\s*(\\])"),
      scope: vec![
        Scope {
            a: 46444148594376762,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521948725430,
            b: 16325548649218048,
        },
    ]),(2, vec![
        Scope {
            a: 47288521948725419,
            b: 16325548649218048,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\["),
      scope: vec![
        Scope {
            a: 47288521948725430,
            b: 16325548649218048,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4010 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4108 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 47288521944400054,
            b: 16325548649218048,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4011 }),
    ]),
      with_prototype: None
    }),
]
} }