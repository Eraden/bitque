
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
      regex: Regex::new("(?=;|(?m:$)|^)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9663 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9578 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([_$\\p{L}][_$\\p{L}\\p{N}]*)\\s*(?:(\\.)|(\\?\\.(?!\\s*[\\p{Nd}])))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632450811,
            b: 42221246506598400,
        },
    ]),(2, vec![
        Scope {
            a: 47288788234665984,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288788293582998,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([_$\\p{L}][_$\\p{L}\\p{N}]*)"),
      scope: vec![
        Scope {
            a: 49259087310291094,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }