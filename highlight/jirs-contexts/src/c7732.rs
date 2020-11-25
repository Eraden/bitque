
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 844931736272896,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844931736272896,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(;|#).*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 51510711072129142,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323038838,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(\\[)(.*?)(\\])"),
      scope: vec![
        Scope {
            a: 46444230161793142,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322121334,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59391945943154688,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629322121334,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(\\s*([\"\']?)(.+?)(\\2)\\s*(=))?\\s*(([\"\']?)(.*?)(\\7))\\s*(;.*)?(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 46445763461775360,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444371892371456,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629372321910,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636731185692672,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629372321910,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629459288182,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 46444947417989120,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 47288629372321910,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 55451480969117814,
            b: 0,
        },
    ]),(9, vec![
        Scope {
            a: 47288629372321910,
            b: 0,
        },
    ]),(10, vec![
        Scope {
            a: 51520013971292278,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }