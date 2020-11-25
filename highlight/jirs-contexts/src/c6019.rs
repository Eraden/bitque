
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
        index: 6125,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(other|macro-(?:params|selfref|defaults)|orphan-labels|number-overflow|gnu-elf-extensions|float-(?:overflow|denorm|underflow|toolong)|user|lock|hle|bnd|zext-reloc|ptr|(?:bad|unknown|not-my)-pragma|unknown-warning|all)(?=\\]|\\s)"),
      scope: vec![
        Scope {
            a: 61925409743962504,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\S+(?=\\])"),
      scope: vec![
        Scope {
            a: 50103314668060885,
            b: 170012569565986903,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\S+"),
      scope: vec![
        Scope {
            a: 50103314668060885,
            b: 170012569565986903,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }