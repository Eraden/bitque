
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
  uses_backrefs: true,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: true,
      regex: Regex::new("(</)(\\1)(>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444230165332170,
            b: 1407374883553280,
        },
        Scope {
            a: 47288629324153014,
            b: 1407374883553280,
        },
    ]),(2, vec![
        Scope {
            a: 46444230165332170,
            b: 1407374883553280,
        },
        Scope {
            a: 59392130632122591,
            b: 56857966770388992,
        },
    ]),(3, vec![
        Scope {
            a: 46444230165332170,
            b: 1407374883553280,
        },
        Scope {
            a: 47288629324153003,
            b: 1407374883553280,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3315 })),
]
} }