
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
        a: 46444882986532864,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444882986532864,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 742 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(::)\\s*)?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*(::)\\s*"),
      scope: vec![
        Scope {
            a: 49258881134362624,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788225622016,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788225622016,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(::)\\s*)?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b"),
      scope: vec![
        Scope {
            a: 49258881134362624,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788225622016,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 46443865079283712,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 3377699720527872,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 639 }),
    ]),
      with_prototype: None
    }),
]
} }