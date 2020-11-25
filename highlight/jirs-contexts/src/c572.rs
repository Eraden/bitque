
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
            a: 48414439024361472,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 446 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 559 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 620 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 619 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bstatic_assert(?=\\s*\\()"),
      scope: vec![
        Scope {
            a: 46445346843000832,
            b: 0,
        },
        Scope {
            a: 52636628119322636,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 447 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*(::)\\s*)?~\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b(?=\\s*(\\(|(?m:$)))"),
      scope: vec![
        Scope {
            a: 46444084150206476,
            b: 0,
        },
        Scope {
            a: 59392130630615473,
            b: 3377699720527872,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788225622016,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 617 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*(\\()(?=\\s*(?!void)\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*[),])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49258881134362624,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46443865079283712,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 3377699720527872,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 449 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 657 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?!struct|union|enum\\s+class|enum\\s+struct|enum|class|template)\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)(?=\\s*\\()"),
      scope: vec![
        Scope {
            a: 46444084145684492,
            b: 0,
        },
        Scope {
            a: 59392130630615404,
            b: 3377699720527872,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 617 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*(::)\\s*\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)(?=\\s*\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444084145684492,
            b: 0,
        },
        Scope {
            a: 59392130630615404,
            b: 3377699720527872,
        },
    ]),(2, vec![
        Scope {
            a: 47288788225622016,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 617 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 576 }),
    ]),
      with_prototype: None
    }),
]
} }