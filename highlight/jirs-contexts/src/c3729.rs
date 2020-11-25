
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 3756 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3771 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3757 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3808 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3772 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3704 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3816 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3709 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(","),
      scope: vec![
        Scope {
            a: 47288620724781056,
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
            a: 50103314683854904,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3705 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3706 })),
]
} }