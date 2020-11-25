
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
      regex: Regex::new("\""),
      scope: vec![
        Scope {
            a: 47288629323956395,
            b: 16044073672507392,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:AND|OR|NOT|IN)\\b"),
      scope: vec![
        Scope {
            a: 52636628114801745,
            b: 310467144124989440,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:ALL|ANY|SOME|NONE)\\b"),
      scope: vec![
        Scope {
            a: 59955110709625935,
            b: 16044073672507392,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:NULL|NIL|SELF|TRUE|YES|FALSE|NO|FIRST|LAST|SIZE)\\b"),
      scope: vec![
        Scope {
            a: 59955110709625935,
            b: 16044073672507392,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:MATCHES|CONTAINS|BEGINSWITH|ENDSWITH|BETWEEN|LIKE)\\b"),
      scope: vec![
        Scope {
            a: 52636628119258193,
            b: 310467144124989440,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:C(ASEINSENSITIVE|I))\\b"),
      scope: vec![
        Scope {
            a: 52636787023873105,
            b: 310467144124989440,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:ANYKEY|SUBQUERY|CAST|TRUEPREDICATE|FALSEPREDICATE)\\b"),
      scope: vec![
        Scope {
            a: 52636787084952655,
            b: 16044073672507392,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\(\\\\|[abefnrtv\'\"?]|[0-3]\\d{,2}|[4-7]\\d?|x[a-zA-Z0-9]+)"),
      scope: vec![
        Scope {
            a: 59955200847315001,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\."),
      scope: vec![
        Scope {
            a: 50103314684903481,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }