
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
        a: 46444328983396513,
        b: 0,
    },
    Scope {
        a: 46444268828360865,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444328983396513,
        b: 0,
    },
    Scope {
        a: 46444268828360865,
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
      regex: Regex::new("\\n"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 10288 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("!"),
      scope: vec![
        Scope {
            a: 52636628114800801,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:all)\\b"),
      scope: vec![
        Scope {
            a: 59955110657196193,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:canonical|final)\\b"),
      scope: vec![
        Scope {
            a: 52636787023085568,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((?i:exec))\\b[ \\t]*(\\\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787023085568,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 55451420828565665,
            b: 0,
        },
        Scope {
            a: 47288629323956406,
            b: 45317471250415616,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10262 }),
        ContextReference::Direct(ContextId { index: 5464 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 10263 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((?i:exec))\\b[ \\t]+"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787023085568,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10264 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 10265 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:(?:original)?host)\\b"),
      scope: vec![
        Scope {
            a: 52636787023085568,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10266 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:(?:local)?user)\\b"),
      scope: vec![
        Scope {
            a: 52636787023085568,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10267 }),
    ]),
      with_prototype: None
    }),
]
} }