
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
        index: 2664,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  # 1., 1.1, 1.1e1, 1.1e-1, 1.e1, 1.e-1 | 1e1, 1e-1\n  (?:0|[1-9][0-9_]*) (?: (\\.) [0-9_]* (?:[Ee](?:[-+]|(?![-+]))[0-9_]*)? | (?:[Ee](?:[-+]|(?![-+]))[0-9_]*) )\n  # .1, .1e1, .1e-1\n  | (\\.) [0-9_]+ (?:[Ee](?:[-+]|(?![-+]))[0-9_]*)?\n)(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 12103423998558208,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397931,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620735397931,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("0[0-9_]+(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 59955089176461666,
            b: 12103423998558208,
        },
        Scope {
            a: 50104723416940898,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(0[Xx])[\\h_]*(n)?(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 12103423998558208,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 12103423998558208,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553259,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(0[Oo])[0-7_]*(n)?(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 59955089176461666,
            b: 12103423998558208,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 12103423998558208,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553259,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(0[Bb])[01_]*(n)?(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 59955089176461741,
            b: 12103423998558208,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 12103423998558208,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553259,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:0|[1-9][0-9_]*)(n|(?!\\.))(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 12103423998558208,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576476553259,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("0[Xx](?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\})))+"),
      scope: vec![
        Scope {
            a: 50103314667667672,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("0[Bb](?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\})))+"),
      scope: vec![
        Scope {
            a: 50103314667667885,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("0(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\})))+"),
      scope: vec![
        Scope {
            a: 50103314667667810,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[1-9](?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\})))+(?:\\.(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\})))*)?"),
      scope: vec![
        Scope {
            a: 50103314667667674,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }