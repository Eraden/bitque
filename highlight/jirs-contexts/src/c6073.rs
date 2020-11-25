
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
      regex: Regex::new("(?i)\\b(v((broadcast|extract|insert|perm2)i128|pmaskmov[dq]|perm([dsq]|p[sd])))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 456559310048526336,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vpbroadcast[bdqw])\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 456559310048591872,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vp(blendd|s[lr]lv[dq]|sravd))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 456559391546540032,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vp?gather[dq][dq]|vgather([dq]|dq)p[ds])\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 456559395841507328,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }