
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
      regex: Regex::new("\\bfriend\\b"),
      scope: vec![
        Scope {
            a: 48414439027245056,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3587 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3708 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3765 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3764 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bstatic_assert(?=\\s*\\()"),
      scope: vec![
        Scope {
            a: 46445346845884416,
            b: 0,
        },
        Scope {
            a: 52636628119322680,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3588 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*(::)\\s*)?~\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b(?=\\s*(\\(|(?m:$)))"),
      scope: vec![
        Scope {
            a: 46444084150206520,
            b: 0,
        },
        Scope {
            a: 59392130630615473,
            b: 15762598695796736,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788228505600,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3762 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*(\\()(?=\\s*(?!void)\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*[),])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49258881137246208,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46443865082167296,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 15762598695796736,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3590 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 657 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?!struct|union|enum\\s+class|enum\\s+struct|enum|class|template)\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)(?=\\s*\\()"),
      scope: vec![
        Scope {
            a: 46444084145684536,
            b: 0,
        },
        Scope {
            a: 59392130630615404,
            b: 15762598695796736,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3762 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*(::)\\s*\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)(?=\\s*\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444084145684536,
            b: 0,
        },
        Scope {
            a: 59392130630615404,
            b: 15762598695796736,
        },
    ]),(2, vec![
        Scope {
            a: 47288788228505600,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3762 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3725 }),
    ]),
      with_prototype: None
    }),
]
} }