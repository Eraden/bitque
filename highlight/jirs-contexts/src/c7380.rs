
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
      regex: Regex::new("(?xi)\n(?:^ \\s*\n  (?: ((?xi:[A-Za-z_][A-Za-z_0-9]* \\s* (?=:))) : )?         # 1 optional select statement label\n  \\s* \\b (select\\s+case) \\b             # 2 select case keyword\n  (?= \\s* \\( )                          #   lookahead to expect opening parenthesis\n  # Can\'t match this stuff due to lack of regex for an expression\n  # \\s* \\(                                #   opening parenthesis\n  #         .*                            #   case expression\n  # \\s* \\)                                #   closing parenthesis\n)\n"),
      scope: vec![
        Scope {
            a: 46453348513415168,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632124632,
            b: 631911322715422720,
        },
    ]),(2, vec![
        Scope {
            a: 52636636840986821,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }