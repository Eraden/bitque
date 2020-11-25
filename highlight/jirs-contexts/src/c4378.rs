
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
      regex: Regex::new("\\b(AUTOLOAD|DESTROY)\\b(?!::)"),
      scope: vec![
        Scope {
            a: 59392130630616307,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(BEGIN|CHECK|END|INIT|UNITCHECK)\\b(?!::)"),
      scope: vec![
        Scope {
            a: 59392130630616306,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?!\\b(?x:\n  # control keywords\n  default|else|elsif|given|if|unless|when|break|caller|continue|die|\n  do|dump|exit|goto|last|next|redo|return|wait|for|foreach|until|while|\n  # declaration keywords\n  package|require|use|no|sub|format|local|my|our|state|\n  # word operators\n  and|or|xor|as|cmp|eq|gt|ge|lt|le|ne|not|x|\n  # quoted like functions (are handled like keywords)\n  m|q|qq|qr|qw|qx|s|tr|y\n)\\b)\\b[_\\p{L}]\\w*\\b"),
      scope: vec![
        Scope {
            a: 59392130630615101,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }