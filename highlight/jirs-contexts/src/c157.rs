
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
      regex: Regex::new("(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521948725256,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 138 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(?<=application )|(?<=app ))(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956232,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 139 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956232,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 140 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\|)[^|\\n]*(\\|)"),
      scope: vec![
        Scope {
            a: 46444088417320960,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629321990152,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629321990152,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(«)(data) (utxt|utf8)([[:xdigit:]]*)(»)(?:\\s+(as)\\s+(?i:Unicode\\s+text))?"),
      scope: vec![
        Scope {
            a: 59955136426475840,
            b: 2251799813685248,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629330509832,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 61925366775349256,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576483500040,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 55451949115965448,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629330509832,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 52636628099268608,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(«)(\\w+)\\b(?=\\s)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629330509832,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 61925366775349256,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 141 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(«)[^»]*(»)"),
      scope: vec![
        Scope {
            a: 50103314673106952,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629330509832,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330509832,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }