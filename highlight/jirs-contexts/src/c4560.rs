
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
      regex: Regex::new("(\\{)\\s*(\\})"),
      scope: vec![
        Scope {
            a: 46445780681949246,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521973629110,
            b: 17451448556060672,
        },
    ]),(2, vec![
        Scope {
            a: 47288521973629099,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4544 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{(?=(?x:\n  \\s+                      # whitespace\n  | [urfb]*\"(?:\\\\.|[^\"])*\" # strings\n  | [urfb]*\'(?:\\\\.|[^\'])*\' # ^\n  | [\\d.ej]+               # numerics\n  | [+*/%@-] | // | and | or # operators\n  | (\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b[ ]*\\.[ ]*)*\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b               # a path\n)*:|\\s*\\*\\*)"),
      scope: vec![
        Scope {
            a: 47288521973629110,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4595 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{(?=(?x:\n  \\s+                      # whitespace\n  | [urfb]*\"(?:\\\\.|[^\"])*\" # strings\n  | [urfb]*\'(?:\\\\.|[^\'])*\' # ^\n  | [\\d.ej]+               # numerics\n  | [+*/%@-] | // | and | or # operators\n  | (\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b[ ]*\\.[ ]*)*\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b               # a path\n)*[,}]|\\s*\\*)"),
      scope: vec![
        Scope {
            a: 47288521953050806,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4598 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{"),
      scope: vec![
        Scope {
            a: 47288522020421814,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4411 }),
    ]),
      with_prototype: None
    }),
]
} }