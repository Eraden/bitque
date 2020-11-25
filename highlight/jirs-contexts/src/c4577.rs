
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
      regex: Regex::new("(?x)\n(\\{)\n  (?: [\\w.\\[\\]]+)?             # field_name\n  (   ! [ars])?                # conversion\n  (   : (?:(?x:\n  (?:.? [<>=^])?     # fill align\n  [ +-]?             # sign\n  \\#?                # alternate form\n  # technically, octal and hexadecimal integers are also supported as \'width\', but rarely used\n  \\d*                # width\n  ,?                 # thousands separator\n  (?:\\.\\d+)?         # precision\n  [bcdeEfFgGnosxX%]? # type\n)|    # format_spec OR\n           [^}%]*%.[^}]*)      # any format-like string\n  )?\n(\\})"),
      scope: vec![
        Scope {
            a: 59955136434012222,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629338046646,
            b: 17451448556060672,
        },
    ]),(2, vec![
        Scope {
            a: 48414439107199038,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955136432046142,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629338046635,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{(?=[^\\}\"\']+\\{[^\"\']*\\})"),
      scope: vec![
        Scope {
            a: 47288629338046646,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4423 }),
    ]),
      with_prototype: None
    }),
]
} }