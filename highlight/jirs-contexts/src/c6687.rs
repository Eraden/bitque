
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
        a: 281882998603776,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281882998603776,
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
      regex: Regex::new("^(ERROR)(\\|)(\\w+)(\\|)(.+)(\\|)(\\d+)(\\|)(\\d+)(\\|)(.+)(?m:$)"),
      scope: vec![
        Scope {
            a: 275573180551069696,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 275573184857440351,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 275564882674253824,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 92606177877098496,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 602075383205986304,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(9, vec![
        Scope {
            a: 602356858182696960,
            b: 0,
        },
    ]),(10, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(11, vec![
        Scope {
            a: 161004094700388352,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(WARNING)(\\|)(\\w+)(\\|)(.+)(\\|)(\\d+)(\\|)(\\d+)(\\|)(.+)(?m:$)"),
      scope: vec![
        Scope {
            a: 275573180551069696,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 275573184865501279,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 275564882674253824,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 92606177877098496,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 602075383205986304,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(9, vec![
        Scope {
            a: 602356858182696960,
            b: 0,
        },
    ]),(10, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(11, vec![
        Scope {
            a: 161004094700388352,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(INFO)(\\|)(\\w+)(\\|)(.+)(\\|)(\\d+)(\\|)(\\d+)(\\|)(.+)(?m:$)"),
      scope: vec![
        Scope {
            a: 275573180551069696,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 275573184916881503,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 275564882674253824,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 92606177877098496,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 602075383205986304,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(9, vec![
        Scope {
            a: 602356858182696960,
            b: 0,
        },
    ]),(10, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(11, vec![
        Scope {
            a: 161004094700388352,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(\\w+)(\\|)(\\w+)(\\|)(.+)(\\|)(\\d+)(\\|)(\\d+)(\\|)(.+)(?m:$)"),
      scope: vec![
        Scope {
            a: 275573180551069696,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 275573184890470495,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 275564882674253824,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 92606177877098496,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 602075383205986304,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(9, vec![
        Scope {
            a: 602356858182696960,
            b: 0,
        },
    ]),(10, vec![
        Scope {
            a: 172263510386868224,
            b: 0,
        },
    ]),(11, vec![
        Scope {
            a: 161004094700388352,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }