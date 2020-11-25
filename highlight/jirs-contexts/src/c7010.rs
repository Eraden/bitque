
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
        a: 59955089306746984,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 59955089306746984,
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
      regex: Regex::new("(?x:\n  (?:[01]\\d|2[0-3])  # hour\n  (:)\n  (?:[0-5]\\d)        # minute\n  (?:\n    (:)\n    (?:[0-5]\\d)      # second (optional)\n    (?:\n      (\\.)\n      \\d{1,4}        # fractional second (not rfc5322)\n    )?\n  )?\n)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620756172904,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620756172904,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288620735397992,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 7012 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7008 })),
]
} }