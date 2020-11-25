
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
      regex: Regex::new("\\["),
      scope: vec![
        Scope {
            a: 47288629323956406,
            b: 44473046320283648,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10222 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10240 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\|)(\\d+)(\\|)([\\w+/]{27}=)(\\|)([\\w+/]{27}=)"),
      scope: vec![
        Scope {
            a: 46444217419696664,
            b: 44473046320283648,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629321400320,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955089176463897,
            b: 44473046320283648,
        },
    ]),(3, vec![
        Scope {
            a: 47288629321400320,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 55451949265977502,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629321400320,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 55451949122125982,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("!"),
      scope: vec![
        Scope {
            a: 52636628114800798,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[*?]"),
      scope: vec![
        Scope {
            a: 46444217419694238,
            b: 0,
        },
        Scope {
            a: 55451949106855936,
            b: 0,
        },
        Scope {
            a: 52636628132429982,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[^,\\[\\s*?]+"),
      scope: vec![
        Scope {
            a: 46444217419694238,
            b: 0,
        },
        Scope {
            a: 55451949106855936,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }