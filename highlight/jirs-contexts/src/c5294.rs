
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
        index: 5277,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 5296 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(\\b\\p{Lu}|\\$)(?:(?:[$\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lo}\\p{Nl}0-9]|_(?=[^[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]]))*(?:_[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+)?))|\\b(?:(?:\\p{Ll}|_+(?=[$\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lo}\\p{Nl}0-9]))(?:(?:[$\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lo}\\p{Nl}0-9]|_(?=[^[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]]))*(?:_[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+)?))(?=\\s*\\()"),
      scope: vec![
        Scope {
            a: 61925366759489536,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5189 }),
    ]),
      with_prototype: None
    }),
]
} }