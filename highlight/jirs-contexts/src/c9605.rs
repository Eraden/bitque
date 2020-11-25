
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
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(?:(async)\\s+)?(function\\b)(?:\\s*(\\*))?(?:(?:\\s+|(?<=\\*))([_$\\p{L}][_$\\p{L}\\p{N}]*))?\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439108182166,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576474128534,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52638522221068438,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 46444204392513686,
            b: 0,
        },
        Scope {
            a: 59392130630615190,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9447 }),
    ]),
      with_prototype: None
    }),
]
} }