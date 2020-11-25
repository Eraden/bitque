
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
        index: 6125,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b0[by](?:[01][01_]*)\\.(?:(?:[01][01_]*)?(?:p[+-]?(?:[0-9][0-9_]*))?\\b)?"),
      scope: vec![
        Scope {
            a: 59955089190487086,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b0[by](?:[01][01_]*)(?:p[+-]?(?:[0-9][0-9_]*))\\b"),
      scope: vec![
        Scope {
            a: 59955089190487086,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b0[oq](?:[0-7][0-7_]*)\\.(?:(?:[0-7][0-7_]*)?(?:p[+-]?(?:[0-9][0-9_]*))?\\b)?"),
      scope: vec![
        Scope {
            a: 59955089185571886,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b0[oq](?:[0-7][0-7_]*)(?:p[+-]?(?:[0-9][0-9_]*))\\b"),
      scope: vec![
        Scope {
            a: 59955089185571886,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:0[dt])?(?:[0-9][0-9_]*)\\.(?:(?:[0-9][0-9_]*)?(?:e[+-]?(?:[0-9][0-9_]*))?\\b)?"),
      scope: vec![
        Scope {
            a: 59955089176658990,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:[0-9][0-9_]*)(?:e[+-]?(?:[0-9][0-9_]*))\\b"),
      scope: vec![
        Scope {
            a: 59955089176658990,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:[0-9][0-9_]*)p(?:[0-9][0-9_]*)?\\b"),
      scope: vec![
        Scope {
            a: 59955089176659521,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b0[xh](?:[[:xdigit:]][[:xdigit:]_]*)\\.(?:(?:[[:xdigit:]][[:xdigit:]_]*)?(?:p[+-]?(?:[0-9][0-9_]*))?\\b)?"),
      scope: vec![
        Scope {
            a: 59955089201169454,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b0[xh](?:[[:xdigit:]][[:xdigit:]_]*)(?:p[+-]?(?:[0-9][0-9_]*))\\b"),
      scope: vec![
        Scope {
            a: 59955089201169454,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\$[0-9]\\_?(?:[[:xdigit:]][[:xdigit:]_]*)?\\.(?:(?:[[:xdigit:]][[:xdigit:]_]*)?(?:p[+-]?(?:[0-9][0-9_]*))?\\b)?"),
      scope: vec![
        Scope {
            a: 59955089201169454,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\$[0-9]\\_?(?:[[:xdigit:]][[:xdigit:]_]*)(?:p[+-]?(?:[0-9][0-9_]*))\\b"),
      scope: vec![
        Scope {
            a: 59955089201169454,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }