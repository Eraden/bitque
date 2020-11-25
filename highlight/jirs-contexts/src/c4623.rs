
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
      regex: Regex::new("([uU]?R)(\"\"\")"),
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
            a: 55451420828565727,
            b: 17451448556060672,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4470 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([bB]R|R[bB])(\"\"\")"),
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
            a: 55451420828565727,
            b: 17451448556060672,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4471 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([uU]?r)(\"\"\")"),
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
            a: 55451420828565727,
            b: 17451448556060672,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4481 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([bB]r|r[bB])(\"\"\")"),
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
            a: 55451420828565727,
            b: 17451448556060672,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4486 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([fF]R|R[fF])(\"\"\")"),
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
            a: 55451420828565727,
            b: 17451448556060672,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4472 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([fF]r|r[fF])(\"\"\")"),
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
            a: 55451420828565727,
            b: 17451448556060672,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4473 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([fF])(\"\"\")"),
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
            a: 55451420828565727,
            b: 17451448556060672,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4475 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([uU]?)(\"\"\")"),
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
            a: 55451420828565727,
            b: 17451448556060672,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4476 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([bB])(\"\"\")"),
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
            a: 55451420828565727,
            b: 17451448556060672,
        },
        Scope {
            a: 47288629323956406,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4480 }),
    ]),
      with_prototype: None
    }),
]
} }