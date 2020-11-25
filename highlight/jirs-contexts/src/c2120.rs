
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
        a: 46444230200328374,
        b: 1407374883553280,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444230200328374,
        b: 1407374883553280,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 2118 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(">"),
      scope: vec![
        Scope {
            a: 47288629324153003,
            b: 1407374883553280,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2074 }),
    ]),
      with_prototype: None
    }),
]
} }