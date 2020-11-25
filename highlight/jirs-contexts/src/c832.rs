
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
      regex: Regex::new("\\{"),
      scope: vec![
        Scope {
            a: 47288629325660342,
            b: 3940649673949184,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 833 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\}"),
      scope: vec![
        Scope {
            a: 46444328935882752,
            b: 0,
        },
        Scope {
            a: 47288629325660331,
            b: 3940649673949184,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 998 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1040 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1041 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1002 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1030 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 997 })),
]
} }