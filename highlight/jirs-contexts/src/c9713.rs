
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
      regex: Regex::new("(?=\\{)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9873 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9872 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9989 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9892 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([_$\\p{L}][_$\\p{L}\\p{N}]*)\\s*(?:(\\.)|(\\?\\.(?!\\s*[\\p{Nd}])))(?=\\s*[_$\\p{L}][_$\\p{L}\\p{N}]*(\\s*\\??\\.\\s*[_$\\p{L}][_$\\p{L}\\p{N}]*)*\\s*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632450811,
            b: 42502721483309056,
        },
    ]),(2, vec![
        Scope {
            a: 47288788234731520,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288788293582999,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([_$\\p{L}][_$\\p{L}\\p{N}]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392186470432919,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9891 })),
]
} }