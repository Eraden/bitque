
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
      regex: Regex::new("([uU]?R)(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475439166,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444217269878784,
            b: 0,
        },
        Scope {
            a: 55451420828565566,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4489 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([bB]R|R[bB])(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475439166,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444217269878784,
            b: 0,
        },
        Scope {
            a: 55451420828565566,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4490 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([uU]?r)(\")(?=\\s*(?:SELECT|INSERT|UPDATE|DELETE|CREATE|REPLACE|ALTER|WITH)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475439166,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444217269878784,
            b: 0,
        },
        Scope {
            a: 55451420828565566,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4498 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([uU]?r)(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475439166,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444217269878784,
            b: 0,
        },
        Scope {
            a: 55451420828565566,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4500 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([bB]r|r[bB])(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475439166,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444217269878784,
            b: 0,
        },
        Scope {
            a: 55451420828565566,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4502 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(R[fF]|[fF]R)(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475439166,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444217290063934,
            b: 0,
        },
        Scope {
            a: 55451420828565566,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4505 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(r[fF]|[fF]r)(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475439166,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444217290063934,
            b: 0,
        },
        Scope {
            a: 55451420828565566,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4491 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([fF])(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475439166,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444217290063934,
            b: 0,
        },
        Scope {
            a: 55451420828565566,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4493 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([uU]?)(\")(?=\\s*(?:SELECT|INSERT|UPDATE|DELETE|CREATE|REPLACE|ALTER|WITH)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475439166,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444217269878784,
            b: 0,
        },
        Scope {
            a: 55451420828565566,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4494 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([uU]?)(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475439166,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444217269878784,
            b: 0,
        },
        Scope {
            a: 55451420828565566,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4496 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([bB])(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475439166,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444217269878784,
            b: 0,
        },
        Scope {
            a: 55451420828565566,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4497 }),
    ]),
      with_prototype: None
    }),
]
} }