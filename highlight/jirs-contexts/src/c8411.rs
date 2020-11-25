
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
      regex: Regex::new("\\\\[nN]"),
      scope: vec![
        Scope {
            a: 59955200847315291,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[cC]|\\\\[rR]"),
      scope: vec![
        Scope {
            a: 59955200847317360,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[lL]"),
      scope: vec![
        Scope {
            a: 59955200847317313,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[fF]"),
      scope: vec![
        Scope {
            a: 59955200847317312,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[tT]"),
      scope: vec![
        Scope {
            a: 59955200847317361,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[vV]"),
      scope: vec![
        Scope {
            a: 59955200847317362,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\\\\""),
      scope: vec![
        Scope {
            a: 59955200847317309,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\\'"),
      scope: vec![
        Scope {
            a: 59955200847317310,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[0-9]+"),
      scope: vec![
        Scope {
            a: 59955200847317363,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[aA]"),
      scope: vec![
        Scope {
            a: 59955200847317364,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[bB]"),
      scope: vec![
        Scope {
            a: 59955200847317130,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[eE]"),
      scope: vec![
        Scope {
            a: 59955200847315185,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[xX][0-9A-Fa-f]{2}"),
      scope: vec![
        Scope {
            a: 59955200847315536,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\\\\\"),
      scope: vec![
        Scope {
            a: 59955200847315722,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }