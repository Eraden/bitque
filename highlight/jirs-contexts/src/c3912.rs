
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 3946 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3896 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3895 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3975 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3923 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3922 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3932 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3935 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3974 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3936 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3892 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3983 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3897 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3891 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(","),
      scope: vec![
        Scope {
            a: 47288620724846592,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\)|\\}"),
      scope: vec![
        Scope {
            a: 50103314683854905,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3893 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3894 })),
]
} }