
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
      regex: Regex::new("DEFINE(?=\\))"),
      scope: vec![
        Scope {
            a: 52636787035472066,
            b: 12385324177031168,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6914 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(R)(?:(&)([a-zA-Z_][a-zA-Z_\\d]{,31})|(\\d+)|(?=\\)))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628241023020,
            b: 27866022694354944,
        },
    ]),(2, vec![
        Scope {
            a: 52636628241023020,
            b: 27866022694354944,
        },
    ]),(3, vec![
        Scope {
            a: 49259087434285100,
            b: 27866022694354944,
        },
    ]),(4, vec![
        Scope {
            a: 49259087434285100,
            b: 27866022694354944,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6914 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<([a-zA-Z_][a-zA-Z_\\d]{,31})>|\'\\g<-1>\'|\\g<-1>"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087346925612,
            b: 27866022694354944,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6914 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\?<?[=!]"),
      scope: vec![
        Scope {
            a: 52636636736061484,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6893 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 6914 }),
    ]),
      with_prototype: None
    }),
]
} }