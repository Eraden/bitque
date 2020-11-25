
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
      regex: Regex::new("(?x)\n  (%[-+ ]?\n    (?:<\\|?|>>?\\|?|><\\|?)  # alignment directions\n  )\n  ((\\()\n    (\\d+)                  # space reserved\n    (?:(,)([lm]?trunc))?   # truncation directions\n  (\\)))\n"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444883029196821,
            b: 0,
        },
        Scope {
            a: 49258881177026581,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444883013993111,
            b: 5910974510923776,
        },
    ]),(3, vec![
        Scope {
            a: 47288521948004534,
            b: 186617999753478144,
        },
    ]),(4, vec![
        Scope {
            a: 59955089176461975,
            b: 5910974510923776,
        },
    ]),(5, vec![
        Scope {
            a: 47288620737626775,
            b: 5910974510923776,
        },
    ]),(6, vec![
        Scope {
            a: 61925409748877975,
            b: 5910974510923776,
        },
    ]),(7, vec![
        Scope {
            a: 47288521948004523,
            b: 186617999753478144,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }