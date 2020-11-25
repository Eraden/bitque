
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
        index: 1314,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(bool|byte|cdouble|cent|cfloat|char|creal|dchar|double|float|idouble|ifloat|int|ireal|long|real|short|ubyte|ucent|uint|ulong|ushort|void|wchar|string|dstring|wstring)\\b"),
      scope: vec![
        Scope {
            a: 48414576463577088,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(\\b|\\.)\\d)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1304 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\')"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1199 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=`|[rxq]?\"|q{)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1320 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(null|true|false|__FILE__|__FILE_FULL_PATH__|__MODULE__|__LINE__|__FUNCTION__|__PRETTY_FUNCTION__|__DATE__|__EOF__|__TIME__|__TIMESTAMP__|__VENDOR__|__VERSION__|__ctfe)\\b"),
      scope: vec![
        Scope {
            a: 59955110638256128,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(this|super)\\b"),
      scope: vec![
        Scope {
            a: 49259061523251200,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\b[\\p{L}_][\\p{L}0-9_]*\\b)|\\."),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1265 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1303 })),
]
} }