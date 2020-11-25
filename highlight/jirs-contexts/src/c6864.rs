
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 6855 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=([\"\'])(?>\\\\.|(?!\\1).)*\\1:)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6784 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6863 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=~w([/|\"\'])(?>\\\\.|(?!\\1).)*\\1a)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6796 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x) (~[a-zA-Z])\\n | ~[a-zA-Z]([^{\\[<(/|\"\'])"),
      scope: vec![
        Scope {
            a: 46444217272303616,
            b: 0,
        },
        Scope {
            a: 48414576475439203,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314795397219,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 50103314795462755,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("~r"),
      scope: vec![
        Scope {
            a: 46444217272303616,
            b: 0,
        },
        Scope {
            a: 48414576475439203,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6829 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("~R"),
      scope: vec![
        Scope {
            a: 46444217272303616,
            b: 0,
        },
        Scope {
            a: 48414576475439203,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6800 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("~[a-z]"),
      scope: vec![
        Scope {
            a: 46444217272303616,
            b: 0,
        },
        Scope {
            a: 48414576475439203,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6820 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("~[A-Z]"),
      scope: vec![
        Scope {
            a: 46444217272303616,
            b: 0,
        },
        Scope {
            a: 48414576475439203,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6828 }),
    ]),
      with_prototype: None
    }),
]
} }