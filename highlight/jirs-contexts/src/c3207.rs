
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 3336 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3306 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3370 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([ ]{,3})(\\d+)(\\.)(\\s)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 114280017426907185,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 114280017426908073,
            b: 13792273858822144,
        },
    ]),(3, vec![
        Scope {
            a: 114280017426908073,
            b: 13792273858822144,
        },
        Scope {
            a: 47288629372518449,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 114280017426907185,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3208 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([ ]{,3})([*+-])(\\s)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 114280017427103793,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 114280017427104681,
            b: 13792273858822144,
        },
        Scope {
            a: 47288629372518449,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 114280017427103793,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3209 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3360 })),
]
} }