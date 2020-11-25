
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9254 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9260 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9281 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9255 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9285 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9263 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9246 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(class|deinit|enum|extension|func|import|init|let|protocol|static|struct|subscript|typealias|var|throws|rethrows)\\b"),
      scope: vec![
        Scope {
            a: 52638212951244800,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(break|case|continue|default|do|else|fallthrough|if|in|for|return|switch|where|while|repeat|catch|guard|defer|try|throw)\\b"),
      scope: vec![
        Scope {
            a: 52636718302625792,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(as|dynamicType|is|new|super|self|Self|Type)\\b"),
      scope: vec![
        Scope {
            a: 52636787026231442,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(associativity|didSet|get|infix|inout|left|mutating|none|nonmutating|operator|override|postfix|precedence|prefix|right|set|unowned((un)?safe)?|weak|willSet)\\b"),
      scope: vec![
        Scope {
            a: 52636787022102528,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }