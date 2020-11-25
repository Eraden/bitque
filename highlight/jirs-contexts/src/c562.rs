
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 633 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\btemplate\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 436 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 673 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 662 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 672 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(private|protected|public)\\s*(:)(?!:)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439024361472,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288521950167052,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(?=(?:~?\\w+|::))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 572 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 585 })),
]
} }