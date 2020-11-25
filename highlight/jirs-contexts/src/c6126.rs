
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
      regex: Regex::new("(?i)\\b(?:[abcd][hl]|[er]?[abcd]x|[er]?(?:di|si|bp|sp)|dil|sil|bpl|spl|r(?:8|9|1[0-5])[bdlw]?)\\b"),
      scope: vec![
        Scope {
            a: 59955110741411437,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:[cdefgs]s)\\b"),
      scope: vec![
        Scope {
            a: 59955110741411490,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:[er]?flags)\\b"),
      scope: vec![
        Scope {
            a: 59955110741411037,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:[er]?ip)\\b"),
      scope: vec![
        Scope {
            a: 59955110741411491,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:cr[02-4])\\b"),
      scope: vec![
        Scope {
            a: 59955110741409982,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:(?:mm|st|fpr)[0-7])\\b"),
      scope: vec![
        Scope {
            a: 59955110741411457,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:[xy]mm(?:[0-9]|1[0-5])|mxcsr)\\b"),
      scope: vec![
        Scope {
            a: 59955110741411492,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:zmm(?:[12]?[0-9]|30|31))\\b"),
      scope: vec![
        Scope {
            a: 59955110741411426,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:bnd(?:[0-3]|cfg[su]|status))\\b"),
      scope: vec![
        Scope {
            a: 59955110741411493,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:(?:[gil]dt)r?|tr)\\b"),
      scope: vec![
        Scope {
            a: 59955110741411494,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:dr[0-367])\\b"),
      scope: vec![
        Scope {
            a: 59955110741411070,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:cr8|dr(?:[89]|1[0-5])|efer|tpr|syscfg)\\b"),
      scope: vec![
        Scope {
            a: 59955110741411461,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:db[0-367]|t[67]|tr[3-7]|st)\\b"),
      scope: vec![
        Scope {
            a: 50104723416875227,
            b: 447545578047864832,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b[xy]mm(?:1[6-9]|2[0-9]|3[01])\\b"),
      scope: vec![
        Scope {
            a: 50104723416875227,
            b: 447545578047864832,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }