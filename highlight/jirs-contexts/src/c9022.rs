
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
      regex: Regex::new("(?=\\))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9123 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9098 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9099 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9138 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9115 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[0-9]+(x)"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 3940649673949184,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787033309198,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9148 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9156 })),
]
} }