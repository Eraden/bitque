
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
        a: 844837246992384,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844837246992384,
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
      regex: Regex::new("^(#!.*)(?m:$)"),
      scope: vec![
        Scope {
            a: 46444466392924256,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*\\b(library|import|export|part of|part)\\b"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 52636787041304672,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6703 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6718 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6720 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6717 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6725 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6721 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6722 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6723 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6726 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6730 })),
]
} }