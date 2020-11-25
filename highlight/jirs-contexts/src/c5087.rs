
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
      regex: Regex::new("(?:\\b(Copy|Send|Sized|Sync|Drop|Fn|FnMut|FnOnce|Box|ToOwned|Clone|PartialEq|PartialOrd|Eq|Ord|AsRef|AsMut|Into|From|Default|Iterator|Extend|IntoIterator|DoubleEndedIterator|ExactSizeIterator|Option|Some|None|Result|Ok|Err|SliceConcatExt|String|ToString|Vec)\\b|(?:(?:[\\p{L}][_\\p{L}\\p{N}]*|_[_\\p{L}\\p{N}]+)\\b))(?=<)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925375349358592,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5049 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(Self|i8|i16|i32|i64|i128|isize|u8|u16|u32|u64|u128|usize|f32|f64|bool|char|str)\\b"),
      scope: vec![
        Scope {
            a: 48414576467247104,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }