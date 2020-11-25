
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
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 47288521944400054,
            b: 20547673299877888,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5163 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(?:(?:[$\\p{Lu}](?:(?:[$\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lo}\\p{Nl}0-9]|_(?=[^[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]]))*(?:_[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+)?)|(?:(?:\\p{Ll}|_+(?=[$\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lo}\\p{Nl}0-9]))(?:(?:[$\\p{Lu}\\p{Ll}\\p{Lt}\\p{Lo}\\p{Nl}0-9]|_(?=[^[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]]))*(?:_[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+)?)))|(?x:\n  [[^:=<@\\x{2190}\\x{21D2}#]&&[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]][\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]*|\n  =[[^>]&&[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]][\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]*|\n  =>[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+|\n  <(?![\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]|[\\p{L}])|\n  <[[^\\-]&&[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]]+|\n  <-[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+|\n  [:@\\x{2190}\\x{21D2}#][\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+\n))|`[^`\\n]+`)"),
      scope: vec![
        Scope {
            a: 49258876843393024,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("_"),
      scope: vec![
        Scope {
            a: 49259061613756489,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }