
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
      regex: Regex::new("(?xi)\n(?:^ \\s*\n  (?: ((?xi:[A-Za-z_][A-Za-z_0-9]* \\s* (?=:))) : )?         # 1 optional do statement label\n  \\s* \\b (do) \\b                        # 2 do keyword\n  \\s* ((?xi:\\b \\d+ \\b))?                 # 3 statement label with comma\n  \\s* ,?                                #   optional comma\n  \\s* \\b (while) \\b                     # 4 while keyword\n  (?= \\s* \\( )                          #   lookahead to expect opening parenthesis\n  # Can\'t use this stuff:\n  # \\s* \\(                                #   opening parenthesis\n  #         .*                            #   logical scalar expression\n  # \\s* \\)                                #   closing parenthesis\n)\n"),
      scope: vec![
        Scope {
            a: 46453219664396288,
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
            a: 52636636703230149,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130768963781,
            b: 0,
        },
        Scope {
            a: 59955089162371072,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636636704147653,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }