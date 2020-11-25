
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
      regex: Regex::new("switch(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 52636636711616905,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2697 }),
        ContextReference::Direct(ContextId { index: 2695 }),
        ContextReference::Direct(ContextId { index: 2568 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("do(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 52636636708537119,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2562 }),
        ContextReference::Direct(ContextId { index: 2561 }),
        ContextReference::Direct(ContextId { index: 2528 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("for(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 52636636708536544,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2592 }),
        ContextReference::Direct(ContextId { index: 2528 }),
        ContextReference::Direct(ContextId { index: 2589 }),
        ContextReference::Direct(ContextId { index: 2588 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("while(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 52636636708536556,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2713 }),
        ContextReference::Direct(ContextId { index: 2528 }),
        ContextReference::Direct(ContextId { index: 2568 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("with(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 52636636717449453,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2714 }),
        ContextReference::Direct(ContextId { index: 2528 }),
        ContextReference::Direct(ContextId { index: 2568 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("if(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 52636636711616746,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2549 }),
        ContextReference::Direct(ContextId { index: 2528 }),
        ContextReference::Direct(ContextId { index: 2568 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("else\\s+if(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 52636636711616920,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2549 }),
        ContextReference::Direct(ContextId { index: 2528 }),
        ContextReference::Direct(ContextId { index: 2568 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("else(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 52636636711616777,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2549 }),
        ContextReference::Direct(ContextId { index: 2528 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("try(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 52636636706177290,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2701 }),
        ContextReference::Direct(ContextId { index: 2528 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("finally(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 52636636706177433,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2587 }),
        ContextReference::Direct(ContextId { index: 2528 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("catch(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 52636636706177426,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 2535 }),
        ContextReference::Direct(ContextId { index: 2528 }),
        ContextReference::Direct(ContextId { index: 2568 }),
    ]),
      with_prototype: None
    }),
]
} }