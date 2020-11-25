
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 7831 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b((?>await|yield))\\b(?:\\s*(\\*))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636701196331,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52638522221068331,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(return)\\b"),
      scope: vec![
        Scope {
            a: 52636636701196331,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(?>if|else)\\b"),
      scope: vec![
        Scope {
            a: 52636636711616555,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(?>catch|finally|throw|try)\\b"),
      scope: vec![
        Scope {
            a: 52636636714958891,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(?>break|continue|do|goto|while|case|default)\\b"),
      scope: vec![
        Scope {
            a: 52636636708536363,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(?>enum|module|public|package|private|interface|protected)\\b"),
      scope: vec![
        Scope {
            a: 52636787110314027,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(debugger)\\b"),
      scope: vec![
        Scope {
            a: 52636787015352320,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }