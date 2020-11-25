
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 3708 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\btemplate\\b"),
      scope: vec![
        Scope {
            a: 46445286716342272,
            b: 0,
        },
        Scope {
            a: 48414576491495480,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3629 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(~?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46445286716342272,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 49258881148452920,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46443865082167296,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 15762598695796736,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3758 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(~?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*(?=<(?:[^(){}&;*^%=<>-]+(?:<(?:[^(){}&;*^%=<>-]+(?:<[^(){}&;*^%=<>-]*>)?)?\\s*>)?)?[^(){}&;*^%=<>-]*(?:\\([^(){}&;*^%=<>-]*\\))?[^(){}&;*^%=<>-]*>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49258881148452920,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3631 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*(::)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087322284088,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788251050040,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3755 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b"),
      scope: vec![
        Scope {
            a: 49259087310291171,
            b: 15762598695796736,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }