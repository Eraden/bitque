
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
        a: 845000455749632,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 845000455749632,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 8711 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*/\\*"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8692 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)^\\s*\n(node|class)\\s+\n((?:[-_A-Za-z0-9\".]+::)*[-_A-Za-z0-9\".]+)\\s* # identifier"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576471310336,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632450251,
            b: 37717646879227904,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8693 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(define)\\s+([a-zA-Z0-9_:]+)\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576474128518,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615174,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629327560886,
            b: 37717646879227904,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8695 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(\\S+)\\s*{\\s*(\\$[a-zA-Z_]+)\\s*:"),
      scope: vec![
        Scope {
            a: 46444204541739142,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576471310336,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630090886,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(\\S+)\\s*{\\s*([\'\"].+[\'\"])\\s*:"),
      scope: vec![
        Scope {
            a: 46444204541739142,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576471310336,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630090886,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(case|if|unless|else|elsif)\\s+(?!::)"),
      scope: vec![
        Scope {
            a: 52636636697460736,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\$?)\"?[a-zA-Z_\\x{7f}-\\x{ff}][a-zA-Z0-9_\\x{7f}-\\x{ff}]*\"?):(?=\\s+|(?m:$))"),
      scope: vec![
        Scope {
            a: 59392130630090886,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8721 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8722 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8708 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(import|include|require|contain)\\s+([::[a-z]]*)\\b\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636717449670,
            b: 37717646879227904,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8697 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b\\w+\\s*(?==>)\\s*"),
      scope: vec![
        Scope {
            a: 59955136426344582,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<={)\\s*\\w+\\s*(?=})"),
      scope: vec![
        Scope {
            a: 59955136567902342,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(escape|gsub|alert|crit|debug|notice|defined|emerg|err|failed|file|generate|info|realize|search|tag|tagged|template|epp|warning)\\b"),
      scope: vec![
        Scope {
            a: 61925255094337536,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }