
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 798 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 744 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 743 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 825 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 774 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 773 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 781 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 782 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 824 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 783 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 741 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 740 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 827 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 745 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 738 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(","),
      scope: vec![
        Scope {
            a: 47288620721963008,
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
            a: 50103314683854861,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }