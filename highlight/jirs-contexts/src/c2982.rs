
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
        index: 2981,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(\\\\)\n(?:\n  # biblatex commands\n  #   http://mirrors.ibiblio.org/CTAN/macros/latex/exptl/biblatex/doc/biblatex.pdf section 3.8\n  # commands with multicite variant\n    (?:[aA]uto|foot|[pP]aren|[sS]mart|super|[tT]ext)cites?\n  | [Cc]ites?\n  | footcitetexts?\n  # text commands\n  | [cC]ite(?:author)\n  | cite(?:title|year|date)\n  | citeurl\n  # special\n  | (?:[aA]|no|full|footfull)cites?\n  | [vV]olcites?\n  | (?:[pPfFsStTaA]|ft)volcites?\n  | (?:[pPf]?n|N)otecite\n  # non-biblatex\n  | [aA]cite\n  | [cC]ite(?:t|p|alt|alp|text|yearpar)\n)\\b\n# For simplicity with using \\b, we match a star for all variants\n\\*?"),
      scope: vec![
        Scope {
            a: 61925255145193509,
            b: 0,
        },
        Scope {
            a: 52636787072172069,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629362032677,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2931 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(\n  (\\\\)\n  (?:eq|c?page|[vV]|auto|name|[cC])?ref\n  \\*?\n)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255096959013,
            b: 0,
        },
        Scope {
            a: 52636787023937573,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629362032677,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2933 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\\\)label)(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255108689957,
            b: 0,
        },
        Scope {
            a: 48414576485662757,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629362032677,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2935 }),
    ]),
      with_prototype: None
    }),
]
} }