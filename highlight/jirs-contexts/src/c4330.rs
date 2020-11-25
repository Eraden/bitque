
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
      regex: Regex::new("(\")((v?)[\\d_](\\.)[\\d_]+(?:(\\.)[\\d_]+)?)(\")"),
      scope: vec![
        Scope {
            a: 46444217269813248,
            b: 0,
        },
        Scope {
            a: 55451420828565565,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956406,
            b: 17169973579350016,
        },
    ]),(2, vec![
        Scope {
            a: 59955089244422205,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629393096765,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288620735397949,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288620735397949,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 47288629323956395,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\')((v?)[\\d_](\\.)[\\d_]+(?:(\\.)[\\d_]+)?)(\')"),
      scope: vec![
        Scope {
            a: 46444217269813248,
            b: 0,
        },
        Scope {
            a: 55451420831973437,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956406,
            b: 17169973579350016,
        },
    ]),(2, vec![
        Scope {
            a: 59955089244422205,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629393096765,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288620735397949,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288620735397949,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 47288629323956395,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(v?)[\\d_](\\.)[\\d_]+(?:(\\.)[\\d_]+)?\\b"),
      scope: vec![
        Scope {
            a: 59955089244422205,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629393096765,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620735397949,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288620735397949,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4275 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4288 })),
]
} }