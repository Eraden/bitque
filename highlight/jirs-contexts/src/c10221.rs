
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
      regex: Regex::new("(principals)(=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787022823424,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130781,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10220 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 10206 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(tunnel)(=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787022823424,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130781,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10220 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 10207 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(expiry-time)|(valid-before))(=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787022823424,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 50104723413204992,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636628111130781,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10220 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 10208 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(permitlisten)(=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787022823424,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130781,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10220 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 10209 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(permitopen)(=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787022823424,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130781,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10220 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 10210 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(from)(=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787022823424,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130781,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10220 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 10211 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(environment)(=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787022823424,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130781,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10220 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 10212 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(command)(=)(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787022823424,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130781,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 55451420828565661,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 44191571343572992,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10213 }),
        ContextReference::Direct(ContextId { index: 5430 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 10214 }),
    )
    }),
]
} }