
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 7231 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[[^\\n\\S]\\n\\);&\\|<>#]\\S*"),
      scope: vec![
        Scope {
            a: 46444882998067557,
            b: 29836347531329536,
        },
        Scope {
            a: 50103314682347626,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 7250 }),
    ]),
      with_prototype: None
    }),
]
} }