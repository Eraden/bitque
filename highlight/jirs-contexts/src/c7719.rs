
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
      regex: Regex::new("(?<=\\s)((?:end)?(?:autoescape|block|embed|filter|for|if|macro|raw|sandbox|set|spaceless|trans|verbatim)|as|do|else|elseif|extends|flush|from|ignore missing|import|include|only|use|with)(?=\\s)"),
      scope: vec![
        Scope {
            a: 52636636696281088,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }