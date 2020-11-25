
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
        a: 844815772155904,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844815772155904,
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
      regex: Regex::new("^(\\s*)(exposed-modules):(?m:$)"),
      scope: vec![],
      captures: Some(vec![(2, vec![
        Scope {
            a: 59955136412975104,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6575 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(\\s*)(build-depends):(?m:$)"),
      scope: vec![],
      captures: Some(vec![(2, vec![
        Scope {
            a: 59955136412975104,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6576 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*([a-zA-Z_-]+)(:)\\s+"),
      scope: vec![
        Scope {
            a: 59391610927972352,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955136412975104,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288702331453440,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(?i)(executable|library|test-suite|benchmark|flag|source-repository|custom-setup)\\s+([^\\s,]+)\\s*(?m:$)"),
      scope: vec![
        Scope {
            a: 59391610927972352,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636726888955904,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 55450961254023168,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(?i)library\\s*(?m:$)"),
      scope: vec![
        Scope {
            a: 52636726888955904,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("--.*(?m:$)"),
      scope: vec![
        Scope {
            a: 51510311580073984,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }