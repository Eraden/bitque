
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
        index: 8688,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("{"),
      scope: vec![
        Scope {
            a: 47288629471609014,
            b: 37436171902517248,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 8669 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<"),
      scope: vec![
        Scope {
            a: 47288629471609014,
            b: 37436171902517248,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 8670 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8680 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8689 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8690 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8681 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8686 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8677 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8687 })),
]
} }