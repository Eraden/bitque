
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
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(?:(\\bexport)\\s+)?(?:\\b(const)\\s+)?\\b(enum)\\s+([_$\\p{L}][_$\\p{L}\\p{N}]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636728328343,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414439033470976,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576486318231,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130632450411,
            b: 42502721483309056,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9733 }),
    ]),
      with_prototype: None
    }),
]
} }