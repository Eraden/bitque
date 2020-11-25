
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
      regex: Regex::new("\\\\[pP]{\\^?(?>C[cfnos]?|L[&lmotu]?|M[cen]?|N[dlo]?|P[cdefios]?|S[ckmo]?|Z[lps]?|X(?>an|ps|sp|wd|uc))}"),
      scope: vec![
        Scope {
            a: 59955136550078600,
            b: 12385324177031168,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[pP][CLMNPSZ]"),
      scope: vec![
        Scope {
            a: 59955136550078600,
            b: 12385324177031168,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[pP]{[\\p{L}]+}"),
      scope: vec![
        Scope {
            a: 59955136550078601,
            b: 12385324177031168,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[pP](?:({})|{([^\\[\\\\(){}|^$.?*+\\n]+*)}?)|(\\\\[pP])"),
      scope: vec![
        Scope {
            a: 59955136550078600,
            b: 12385324177031168,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314796773420,
            b: 27866022694354944,
        },
    ]),(2, vec![
        Scope {
            a: 50103314796773420,
            b: 27866022694354944,
        },
    ]),(3, vec![
        Scope {
            a: 50103314796773420,
            b: 27866022694354944,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }