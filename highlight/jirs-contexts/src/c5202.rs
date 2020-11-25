
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
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@)((?:[$\\p{Lu}](?:(?:[$\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lo}\\p{Nl}0-9]|_(?=[^[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]]))*(?:_[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+)?)|(?:(?:\\p{Ll}|_+(?=[$\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lo}\\p{Nl}0-9]))(?:(?:[$\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lo}\\p{Nl}0-9]|_(?=[^[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]]))*(?:_[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+)?))))(\\.)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445080559026176,
            b: 0,
        },
        Scope {
            a: 47288629337129033,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46445080565186633,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 46445080565186633,
            b: 0,
        },
        Scope {
            a: 47288788229619712,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5110 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@)((?:[$\\p{Lu}](?:(?:[$\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lo}\\p{Nl}0-9]|_(?=[^[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]]))*(?:_[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+)?)|(?:(?:\\p{Ll}|_+(?=[$\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lo}\\p{Nl}0-9]))(?:(?:[$\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lo}\\p{Nl}0-9]|_(?=[^[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]]))*(?:_[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+)?))))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445080559026176,
            b: 0,
        },
        Scope {
            a: 47288629337129033,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46445080565186633,
            b: 0,
        },
        Scope {
            a: 49259830326132736,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5203 }),
    ]),
      with_prototype: None
    }),
]
} }