
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
      regex: Regex::new(":(ascii|alnum|alpha|blank|cntrl|digit|graph|lower|print|punct|space|upper|word|xdigit):"),
      scope: vec![
        Scope {
            a: 59955136494698540,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\]"),
      scope: vec![
        Scope {
            a: 52636636704866348,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(\\\\(?:[tnrfae]|[0-7]{3}|x\\{\\h{1,7}\\}|x\\h\\h|c\\d+)|\\\\.|(?!\\\\-)[^\\]])-(\\\\(?:[tnrfae]|[0-7]{3}|x\\{\\h{1,7}\\}|x\\h\\h|c\\d+)|\\\\.|[^\\]]))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4741 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4756 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4772 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4758 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4757 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("&&"),
      scope: vec![
        Scope {
            a: 52636628186497068,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }