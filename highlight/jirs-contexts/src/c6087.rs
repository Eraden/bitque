
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
      regex: Regex::new("(?i)\\b(cmp(n?(eq|lt|le)|(un)?ord)[ps][ds])\\b"),
      scope: vec![
        Scope {
            a: 52642976169657988,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(v?pclmul([hl]q[hl]q|[hl]qh)dq)\\b"),
      scope: vec![
        Scope {
            a: 52642976165594694,
            b: 452330287574024192,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vcmp(eq(_(os|uq|us))?|neq(_(oq|os|us))?|[gl][et](_oq)?|n[gl][et](_uq)?|(un)?ord(_s)?|false(_os)?|true(_us)?)[ps][ds])\\b"),
      scope: vec![
        Scope {
            a: 52642976165594694,
            b: 88101667710435328,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(vpcom(n?eq|[gl][et]|false|true)(b|uw))\\b"),
      scope: vec![
        Scope {
            a: 52642976166119045,
            b: 470070181838716928,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }