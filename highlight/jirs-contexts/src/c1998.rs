
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
      regex: Regex::new("/"),
      scope: vec![
        Scope {
            a: 47288629323956268,
            b: 48132362751442944,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\/"),
      scope: vec![
        Scope {
            a: 59955200847314977,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2019 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2045 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2033 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\$\\{"),
      scope: vec![
        Scope {
            a: 47288521949642785,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1999 }),
    ]),
      with_prototype: None
    }),
]
} }