
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
        a: 46445780657963029,
        b: 7599824371187712,
    },
    Scope {
        a: 46445252354310171,
        b: 0,
    },
    Scope {
        a: 55451949097877531,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46445780657963029,
        b: 7599824371187712,
    },
    Scope {
        a: 46445252354310171,
        b: 0,
    },
    Scope {
        a: 55451949097877531,
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
      regex: Regex::new("\\.?\\n"),
      scope: vec![
        Scope {
            a: 46445780657963029,
            b: 7599824371187712,
        },
        Scope {
            a: 47288689469292565,
            b: 7599824371187712,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[\\\\/]"),
      scope: vec![
        Scope {
            a: 47288620749815829,
            b: 7599824371187712,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1812 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[*?]"),
      scope: vec![
        Scope {
            a: 50103314682347541,
            b: 7599824371187712,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }