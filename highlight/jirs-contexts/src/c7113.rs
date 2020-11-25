
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
      regex: Regex::new("(?=[\\n\\)#]|;|&(?![|>])|(?:(?:[0-9]+)?(?:[<>]|>>)|\\^\\^?|&)?\\||(?:[0-9]+)?(?:[<>]|>>)|\\^\\^?|&>>?)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(function)\\s+"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46444882999640170,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52636636711616618,
            b: 0,
        },
        Scope {
            a: 46444217286852714,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7114 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=[^\\n\\S])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7118 }),
    ]),
      with_prototype: None
    }),
]
} }