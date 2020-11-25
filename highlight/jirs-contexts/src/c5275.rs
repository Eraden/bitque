
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
      regex: Regex::new(";"),
      scope: vec![
        Scope {
            a: 47288689445371904,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  [[^:=<@\\x{2190}\\x{21D2}#]&&[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]][\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]*|\n  =[[^>]&&[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]][\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]*|\n  =>[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+|\n  <(?![\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]|[\\p{L}])|\n  <[[^\\-]&&[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]]+|\n  <-[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+|\n  [:@\\x{2190}\\x{21D2}#][\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+\n)(?=[\\(\\[])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5290 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  [[^:=<@\\x{2190}\\x{21D2}#]&&[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]][\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]*|\n  =[[^>]&&[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]][\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]*|\n  =>[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+|\n  <(?![\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]|[\\p{L}])|\n  <[[^\\-]&&[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]]+|\n  <-[\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+|\n  [:@\\x{2190}\\x{21D2}#][\\p{Sm}\\p{So}[[^\\w\\[\\]\\(\\)\\{\\}\'\";,.`_\\s]&&[\\x{20}-\\x{7E}]]]+\n)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }