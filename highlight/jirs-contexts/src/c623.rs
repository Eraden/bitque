
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
      regex: Regex::new("(?x:\n  \\b(?:\\d(?:[\\d\']*\\d)?)\n  (?:\n    (?:\n      (\\.)\n      (?:\n        # 1.1, 1.1e1, 1.1e-1, 1.1f, 1.1e1f, 1.1e-1f, 1.1L, 1.1e1L, 1.1e-1L\n        (?:\\d(?:[\\d\']*\\d)?) (?:[eE][-+]??(?:\\d(?:[\\d\']*\\d)?))?\n        # 1.e1, 1.e-1, 1.e1f, 1.e-1f, 1.e1L, 1.e-1L\n        | (?:[eE][-+]??(?:\\d(?:[\\d\']*\\d)?))\n        # 1., 1.f, 1.L # but not `..`\n        | (?!\\.)\n      )\n      # 1e1 1e1f 1e1L\n      | (?:[eE][-+]??(?:\\d(?:[\\d\']*\\d)?))\n    ) ((?:[a-zA-Z_][\\p{L}\\p{N}_]*|(?=[^\\p{L}\\p{N}_\'])))?\n    # 1f\n    | ([fF])\n  )\n  # .1, .1e1, .1e-1, .1f, .1e1f, .1e-1f, .1L, .1e1L, .1e-1L\n  | (\\.) (?:\\d(?:[\\d\']*\\d)?) (?:[eE][-+]??(?:\\d(?:[\\d\']*\\d)?))? ((?:[a-zA-Z_][\\p{L}\\p{N}_]*|(?=[^\\p{L}\\p{N}_\'])))?\n)"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 3377699720527872,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397900,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553228,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576476553228,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288620735397900,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 48414576476553228,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b0[xX](?=[\\p{L}\\p{N}_\'\'.]+?[pP])"),
      scope: vec![
        Scope {
            a: 47288629325070764,
            b: 3377699720527872,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 510 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b0[bB]"),
      scope: vec![
        Scope {
            a: 47288629325070764,
            b: 3377699720527872,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 511 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b0[xX]"),
      scope: vec![
        Scope {
            a: 47288629325070764,
            b: 3377699720527872,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 512 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b0(?=[\\d\'\'])"),
      scope: vec![
        Scope {
            a: 47288629325070764,
            b: 3377699720527872,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 513 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b\\d+"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 514 }),
    ]),
      with_prototype: None
    }),
]
} }