
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
      regex: Regex::new("(?x)\n%\n  ( \\( (\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b) \\) )? # mapping key\n  \\#?            # alternate form\n  0?             # pad with zeros\n  \\-?            # left-adjust\n  \\ ?            # implicit sign\n  [+-]?          # sign\n  (\\d*|\\*)       # width\n  (\\. (\\d*|\\*))? # precision\n  [hlL]?         # length modifier (but ignored)\n  [acdeEfFgGiorsuxX%]"),
      scope: vec![
        Scope {
            a: 59955136434012222,
            b: 0,
        },
    ],
      captures: Some(vec![(2, vec![
        Scope {
            a: 49259087319007294,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:%(?:[aAwdbBGmyYHIpMSfzZjuUVWcxX%]|-[dmHIMSj]))"),
      scope: vec![
        Scope {
            a: 59955136434012222,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{\\{|\\}\\}"),
      scope: vec![
        Scope {
            a: 59955200847315006,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4577 })),
]
} }