
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
      regex: Regex::new("(?x)^\n(\\[)  # in square brackets\n(template\\s*=\\s*)?(\")?  # might start with template-equals and might have template name in quotes\n(\nsect\\d|abstract|preface|colophon|dedication|glossary|bibliography|synopsis|appendix|index # fixed list of known templates\n)\n(\".*(\\])|(\\]))  # either close the quote (and perhaps go on) and close the bracket, or close the bracket immediately\n\\s*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 49258876942155860,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629414592694,
            b: 23643898043695104,
        },
    ]),(4, vec![
        Scope {
            a: 46444230254264404,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 47288629414592683,
            b: 23643898043695104,
        },
    ]),(7, vec![
        Scope {
            a: 47288629414592683,
            b: 23643898043695104,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }