
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
      regex: Regex::new("\\b(base2dec|bin2dec|cast|cell2mat|cell2struct|cellstr|char|dec2base|dec2bin|dec2hex|hex2dec|hex2num|int2str|mat2cell|mat2str|num2cell|native2unicode|num2hex|num2str|persistent|str2double|str2func|str2mat|str2num|struct2cell|unicode2native)\\b"),
      scope: vec![
        Scope {
            a: 48414439026917376,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }