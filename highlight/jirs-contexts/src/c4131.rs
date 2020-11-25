
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
      regex: Regex::new("\\\\[0-7]{1,3}"),
      scope: vec![
        Scope {
            a: 59955200847315298,
            b: 16325548649218048,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\x[0-9A-Fa-f]{1,2}"),
      scope: vec![
        Scope {
            a: 59955200847315536,
            b: 16325548649218048,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\u\\{[0-9A-Fa-f]+\\}"),
      scope: vec![
        Scope {
            a: 59955200847316088,
            b: 16325548649218048,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[nrt\\\\\\$\\\"]"),
      scope: vec![
        Scope {
            a: 59955200847315002,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(?=\\$.*?\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629333590074,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4057 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$\\{)\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b(\\})"),
      scope: vec![
        Scope {
            a: 49259087295807488,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514490,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629322514490,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$\\{)(?=.*?\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514490,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4058 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  (?=\n    \\$\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\n    (?:\n        ->\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\n        |\n        \\[ ( \\d+ | \\$?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b ) \\]\n    )\n  )\n)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4059 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4156 })),
]
} }