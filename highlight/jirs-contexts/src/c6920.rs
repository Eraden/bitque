
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
      regex: Regex::new("\\)"),
      scope: vec![
        Scope {
            a: 50103314741985324,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\(\\?#"),
      scope: vec![
        Scope {
            a: 47288629323038902,
            b: 12385324177031168,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6896 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 52636636696215596,
            b: 27866022694354944,
        },
        Scope {
            a: 47288629318582454,
            b: 12385324177031168,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6897 }),
    ]),
      with_prototype: None
    }),
]
} }