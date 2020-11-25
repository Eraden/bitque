
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
      regex: Regex::new("(?:builtin|command|exec)\\b(?!\\s*(?:[\\n\\)#]|;|&(?![|>])|(?:(?:[0-9]+)?(?:[<>]|>>)|\\^\\^?|&)?\\||(?:[0-9]+)?(?:[<>]|>>)|\\^\\^?|&>>?|-))"),
      scope: vec![
        Scope {
            a: 46444882999640170,
            b: 0,
        },
        Scope {
            a: 61925255092502528,
            b: 0,
        },
        Scope {
            a: 46444217286852714,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7093 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("time\\b"),
      scope: vec![
        Scope {
            a: 46444882999640170,
            b: 0,
        },
        Scope {
            a: 61925255092502528,
            b: 0,
        },
        Scope {
            a: 46444217286852714,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7094 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(not|!)[^\\n\\S](?!\\s*[\\n\\)#;-]))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7095 }),
    ]),
      with_prototype: None
    }),
]
} }