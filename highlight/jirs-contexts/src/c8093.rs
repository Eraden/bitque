
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
      regex: Regex::new("(\\()(reference|inline|less|css|once|multiple|optional)(\\))"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46445449949675643,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629318582454,
            b: 34621422135410688,
        },
    ]),(2, vec![
        Scope {
            a: 59955136415072256,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582443,
            b: 34621422135410688,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8265 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8277 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8278 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8306 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8328 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8308 })),
]
} }