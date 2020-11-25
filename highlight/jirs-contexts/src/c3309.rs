
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
      regex: Regex::new("(<)((?:https?|ftp)://.*?)(>)"),
      scope: vec![
        Scope {
            a: 46443487184551985,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629312815286,
            b: 13792273858822144,
        },
    ]),(2, vec![
        Scope {
            a: 114280588597985329,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629312815275,
            b: 13792273858822144,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(((https|http|ftp)://)|www\\.)[\\w-]+(\\.[\\w-]+)+"),
      scope: vec![
        Scope {
            a: 114280588597986214,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3203 }),
    ]),
      with_prototype: None
    }),
]
} }