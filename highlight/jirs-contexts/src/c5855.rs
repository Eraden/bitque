
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
      regex: Regex::new("(&)([^\\s\\[\\]/{/},]+)(\\S+)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636703949637,
            b: 22799473113563136,
        },
        Scope {
            a: 47288629365899345,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130633761605,
            b: 22799473113563136,
        },
    ]),(3, vec![
        Scope {
            a: 50103314669372229,
            b: 22799473113563136,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n    ! < (?x: %[0-9A-Fa-f]{2} | [0-9A-Za-z\\-#;/?:@&=+$,_.!~*\'()\\[\\]] )+ >\n  | (?:!(?:[0-9A-Za-z\\-]*!)?) (?x: %[0-9A-Fa-f]{2} | [0-9A-Za-z\\-#;/?:@&=+$_.~*\'()] )+\n  | !\n)(?=\\ |\\t|(?m:$))"),
      scope: vec![
        Scope {
            a: 48414576560177233,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\S+"),
      scope: vec![
        Scope {
            a: 50103314751291473,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }