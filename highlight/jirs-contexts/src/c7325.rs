
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
      regex: Regex::new("(?xi)\n(?:^ \\s*\n  (?: ((?xi:[A-Za-z_][A-Za-z_0-9]* \\s* (?=:))) : )?         # optional do statement label\n  \\s* \\b (do \\s+ concurrent|do) \\b      # do keyword\n  \\s* ((?xi:\\b \\d+ \\b))?                 # statement label with comma\n  \\s* (?:\n        ,?                              # optional comma\n        \\s* ([A-Za-z_][A-Za-z_0-9]*)                 # loop variable name\n        \\s* (=)                         # equals sign\n        # Can\'t match this stuff due to lack of regex for an expression\n        # \\s* an_expr_                  # expression 1\n        # \\s* ,                         # comma\n        # \\s* an_expr_                  # expression 2\n        # (\n        #   \\s* ,\n        #   \\s* an_expr_                # expression 3\n        # )?\n      )?\n)\n"),
      scope: vec![
        Scope {
            a: 46453211074461696,
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
            a: 49267965136535552,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 52636628246988997,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }