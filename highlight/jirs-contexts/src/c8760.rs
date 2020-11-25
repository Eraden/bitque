
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
        a: 845004750716928,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 845004750716928,
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
      regex: Regex::new("^\\s*\\b(module)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787021381632,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8725 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(import)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787021381632,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8726 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*\\b(class)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787021381632,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8728 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*\\b(instance)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787021381632,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8729 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*([a-z_][\\w\\d\\\']*|\\(.+\\))\\s*(::)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130630615175,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628259831943,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8730 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(foreign)\\s+(import)\\s+(data)\\s+(.+?)\\s+(::)"),
      scope: vec![
        Scope {
            a: 46445763615064361,
            b: 37999121855938560,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787021381632,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787021381632,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636787021381632,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130630617497,
            b: 37999121855938560,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(foreign)\\s+(import)\\s+(.+?)\\s+"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787021381632,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787021381632,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130630617497,
            b: 37999121855938560,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8731 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8738 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8792 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8739 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8770 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8800 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8771 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8767 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8772 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8735 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8742 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8744 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8747 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8750 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8754 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8757 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8761 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8765 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8786 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8789 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8795 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8798 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8802 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8805 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8740 })),
]
} }