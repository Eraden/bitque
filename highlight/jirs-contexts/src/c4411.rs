
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
      regex: Regex::new("\\}"),
      scope: vec![
        Scope {
            a: 47288522020421803,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4544 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(?x:\n  \\s+                      # whitespace\n  | [urfb]*\"(?:\\\\.|[^\"])*\" # strings\n  | [urfb]*\'(?:\\\\.|[^\'])*\' # ^\n  | [\\d.ej]+               # numerics\n  | [+*/%@-] | // | and | or # operators\n  | (\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b[ ]*\\.[ ]*)*\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b               # a path\n)*:|\\s*\\*\\*)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4595 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(?x:\n  \\s+                      # whitespace\n  | [urfb]*\"(?:\\\\.|[^\"])*\" # strings\n  | [urfb]*\'(?:\\\\.|[^\'])*\' # ^\n  | [\\d.ej]+               # numerics\n  | [+*/%@-] | // | and | or # operators\n  | (\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b[ ]*\\.[ ]*)*\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b               # a path\n)*[,}]|\\s*\\*)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4598 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(","),
      scope: vec![
        Scope {
            a: 47288620737298494,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4598 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4586 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(":"),
      scope: vec![
        Scope {
            a: 47288620757876985,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4596 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4593 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4570 })),
]
} }