
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
        index: 5074,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(\\d[\\d_]*)(i8|i16|i32|i64|i128|isize|u8|u16|u32|u64|u128|usize)?\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461530,
            b: 20266198323167232,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553288,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0x[\\h_]*\\h[\\h_]*)(i8|i16|i32|i64|i128|isize|u8|u16|u32|u64|u128|usize)?\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461528,
            b: 20266198323167232,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553288,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0o[0-7_]*[0-7][0-7_]*)(i8|i16|i32|i64|i128|isize|u8|u16|u32|u64|u128|usize)?\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461666,
            b: 20266198323167232,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553288,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0b[0-1_]*[0-1][0-1_]*)(i8|i16|i32|i64|i128|isize|u8|u16|u32|u64|u128|usize)?\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461741,
            b: 20266198323167232,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553288,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }