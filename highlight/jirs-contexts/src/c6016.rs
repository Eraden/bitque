
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
  prototype: Some(
    ContextId {
        index: 6125,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:(?:80|[1-6])86|pentium|ppro|p[234]|katmai|willamette|prescott|(?:x|ia)64)\\b"),
      scope: vec![
        Scope {
            a: 61925409743962197,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6066 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6103 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6067 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6068 })),
]
} }