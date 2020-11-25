
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
  prototype: Some(
    ContextId {
        index: 399,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(true|false|null)\\b"),
      scope: vec![
        Scope {
            a: 59955110637928448,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\'\\\'"),
      scope: vec![
        Scope {
            a: 50103314681561099,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\')((?:\\\\[abfnrtv\"\'\\\\]|(?:\\\\u\\h{4}|\\\\U\\h{8})|\\\\x[0-9a-fA-F]{1,4}|\\\\[0-9]{1,3}))(\')"),
      scope: vec![
        Scope {
            a: 59955200832241664,
            b: 0,
        },
    ],
      captures: Some(vec![(2, vec![
        Scope {
            a: 59955200847314955,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\').(\')"),
      scope: vec![
        Scope {
            a: 59955200832241664,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\')[^\']+(\')"),
      scope: vec![
        Scope {
            a: 50103314681626635,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(0[xX])[\\h_]*\\h"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 3096224743817216,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(0[bB])[01_]*[01]"),
      scope: vec![
        Scope {
            a: 59955089176461741,
            b: 3096224743817216,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:[\\d_]*\\d)(?:(?:(?:(\\.)(?:[\\d_]*\\d))(?:[eE][-+]??(?:[\\d_]*\\d))?|(?:[eE][-+]??(?:[\\d_]*\\d)))([fFdDmM])?|([fFdDmM]))"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 3096224743817216,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397899,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553227,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576476553227,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:0|[1-9](?:[\\d_]*\\d)?)([uU][lL]?|[lL][uU]?)?"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 3096224743817216,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576476553227,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\""),
      scope: vec![
        Scope {
            a: 47288629323956406,
            b: 3096224743817216,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 401 }),
    ]),
      with_prototype: None
    }),
]
} }