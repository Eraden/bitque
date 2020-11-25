
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 59392130632122880,
        b: 1407374883553280,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 59392130632122880,
        b: 1407374883553280,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=[^[^\\t\\n\\f /<>]])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  # https://html.spec.whatwg.org/multipage/custom-elements.html#custom-elements-core-concepts\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n)+"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[^\\t\\n\\f /<>]"),
      scope: vec![
        Scope {
            a: 50103314703581189,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }