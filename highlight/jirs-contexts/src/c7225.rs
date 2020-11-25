
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
      regex: Regex::new("(begin|while|if|for|switch|function)\\s*([&\\|<>]+)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444882999640170,
            b: 0,
        },
        Scope {
            a: 49258881140523008,
            b: 0,
        },
        Scope {
            a: 46444217286852714,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 50103314665963626,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(begin)\\s*(\\))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444882999640170,
            b: 0,
        },
        Scope {
            a: 49258881140523008,
            b: 0,
        },
        Scope {
            a: 46444217286852714,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 50103314665963626,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("begin(?=\\s*(?m:$)|\\s*[\\n;]|\\s+[^\\s-])"),
      scope: vec![
        Scope {
            a: 46444882999640170,
            b: 0,
        },
        Scope {
            a: 52636636711616618,
            b: 0,
        },
        Scope {
            a: 46444217286852714,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7103 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=while\\s+[^\\n\\)#;-])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7104 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=if\\s+[^\\n\\)#;-])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7121 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=for\\s+[^\\n\\)#;-])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7122 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=switch\\s+[^\\n\\)#;-])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7106 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=function\\s+[^\\n\\)#;-])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7112 }),
    ]),
      with_prototype: None
    }),
]
} }