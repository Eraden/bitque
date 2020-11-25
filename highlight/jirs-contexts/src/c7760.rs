
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
      regex: Regex::new("(</)(\\2)(>)|(/>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444230305185792,
            b: 0,
        },
        Scope {
            a: 47288629324153014,
            b: 663436520107016192,
        },
    ]),(2, vec![
        Scope {
            a: 46444230305185792,
            b: 0,
        },
        Scope {
            a: 59392130632124725,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 46444230305185792,
            b: 0,
        },
        Scope {
            a: 47288629324153003,
            b: 663436520107016192,
        },
    ]),(4, vec![
        Scope {
            a: 46444230305185792,
            b: 0,
        },
        Scope {
            a: 47288629324153003,
            b: 663436520107016192,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7820 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7816 })),
]
} }