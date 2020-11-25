
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
      regex: Regex::new("\\b(import)\\s+(?:(typealias|struct|class|enum|protocol|var|func)\\s+)?((?:\\B\\$[0-9]+|\\b[\\w^\\d][\\w\\d]*\\b|\\B`[\\w^\\d][\\w\\d]*`\\B|[/=\\-+!*%<>&|\\^~.]+)(?:\\.(?:\\B\\$[0-9]+|\\b[\\w^\\d][\\w\\d]*\\b|\\B`[\\w^\\d][\\w\\d]*`\\B|[/=\\-+!*%<>&|\\^~.]+))*)"),
      scope: vec![
        Scope {
            a: 46445256657469440,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787041304722,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414439033143296,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925375394644407,
            b: 41095346599755776,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }