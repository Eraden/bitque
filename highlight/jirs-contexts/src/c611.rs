
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 559 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\btemplate\\b"),
      scope: vec![
        Scope {
            a: 46445286713458688,
            b: 0,
        },
        Scope {
            a: 48414576491495436,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 488 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\boperator\\s*(?:[-+*/%^&|~!=<>]|[-+*/%^&|=!<>]=|<<=?|>>=?|&&|\\|\\||\\+\\+|--|,|->\\*?|\\(\\)|\\[\\]|\"\"\\s*\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b))\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46445286713458688,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 49258881148452876,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46443865079283712,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 3377699720527872,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 614 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(~?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46445286713458688,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 49258881148452876,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46443865079283712,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 3377699720527872,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 614 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(~?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*(?=<(?:[^(){}&;*^%=<>-]+(?:<(?:[^(){}&;*^%=<>-]+(?:<[^(){}&;*^%=<>-]*>)?)?\\s*>)?)?[^(){}&;*^%=<>-]*(?:\\([^(){}&;*^%=<>-]*\\))?[^(){}&;*^%=<>-]*>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49258881148452876,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 490 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*(::)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087322284044,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788251049996,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 611 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b"),
      scope: vec![
        Scope {
            a: 49259087310291171,
            b: 3377699720527872,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }