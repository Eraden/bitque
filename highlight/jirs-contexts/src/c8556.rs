
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
        a: 282029027491840,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 282029027491840,
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
      regex: Regex::new("(?x)\n^(\\*+)               # leading stars\n\\s*([A-Z_]{2,})?     # todo keywords\n\\s*(\\[\\#[A-Ca-c]\\])? # priority\n\\s*(?=\\s+[A-Za-z0-9]+)  # expected heading text"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629353709697,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787039076481,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955136566591617,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8536 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^-{5,}(?m:$)"),
      scope: vec![
        Scope {
            a: 46444195799433216,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*([-+]|\\s+\\*)\\s+(.*?)\\s+::"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628116701313,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 114281679530623105,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8537 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*?([-+]|\\s+\\*|\\d+[).])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628116701313,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8543 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)^#\\+(BEGIN_(QUOTE|VERSE|CENTER))(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636703293569,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8544 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^#[^+].*(?m:$)"),
      scope: vec![
        Scope {
            a: 51510711032873089,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^:(PROPERTIES):(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636848128129,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8545 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^:(.+):(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636848128129,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8546 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)#\\+(BEGIN_LaTeX)(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636703293569,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8547 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)#\\+(BEGIN_HTML)(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636703293569,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8548 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)#\\+(BEGIN_SRC)\\s+(python)\\s*(.*)(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636703293569,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955136421363841,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 55451536939024513,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8549 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)#\\+(BEGIN_SRC)\\s+(ruby)\\s*(.*)(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636703293569,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955136421363841,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 55451536939024513,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8550 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)#\\+(BEGIN_SRC)\\s+((?:emacs-)?lisp)\\s*(.*)(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636703293569,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955136421363841,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 55451536939024513,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8538 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)#\\+(BEGIN_SRC)\\s+(sh)\\s*(.*)(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636703293569,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955136421363841,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 55451536939024513,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8539 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)#\\+(BEGIN_(SRC|EXAMPLE))(?:\\s+([\\w-]+)?\\s*(.*))?(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636703293569,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955136421363841,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 55451536939024513,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8540 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("#\\+(OPTIONS):"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636719743105,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8541 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("#\\+(CALL):\\s+[A-Za-z-]+(?:\\[([^\\]]*)\\])?\\(?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636719743105,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 55451536939024513,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8542 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^#\\+([a-zA-Z_-]+): ?(.*)(?m:$)"),
      scope: vec![
        Scope {
            a: 46445406980210688,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636719743105,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 55451949127565441,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8554 })),
]
} }