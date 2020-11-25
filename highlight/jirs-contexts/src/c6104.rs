
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
      regex: Regex::new("(?i)\\b(?:strict|nosplit|near|far|abs|rel)\\b"),
      scope: vec![
        Scope {
            a: 48414439029145687,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:[ao](?:16|32|64))\\b"),
      scope: vec![
        Scope {
            a: 48414439056932949,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:rep(?:n?[ez])?|lock|xacquire|xrelease|(?:no)?bnd)\\b"),
      scope: vec![
        Scope {
            a: 48414439056932949,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("{(vex[23]|evex)}"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439056934559,
            b: 23925746682560512,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("{(k[1-7])}"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439131316309,
            b: 24488322973827072,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("{(1to(?:8|16))}"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439065387093,
            b: 24488322973827072,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("{(z|(?:r[nudz]-)?sae)}"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439134724181,
            b: 24488322973827072,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }