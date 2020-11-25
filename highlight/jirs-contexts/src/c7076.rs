
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
      regex: Regex::new("\\b(private|to|public|internal|function|yield!|yield|class|exception|match|delegate|of|new|in|as|if|then|else|elif|for|begin|end|inherit|do|let\\!|return\\!|return|interface|with|abstract|property|union|enum|member|try|finally|and|when|or|use|use\\!|struct|while|mutable)(?!\')\\b"),
      scope: vec![
        Scope {
            a: 52636271616458752,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(&&&|\\|\\|\\||\\^\\^\\^|~~~|<<<|>>>|\\|>|\\->|\\<\\-|:>|:\\?>|:|\\[|\\]|\\;|<>|=|@|\\|\\||&&|{|}|\\||_|\\.\\.|\\,|\\+|\\-|\\*|\\/|\\^|\\!|\\>|\\>\\=|\\>\\>|\\<|\\<\\=|\\(|\\)|\\<\\<)"),
      scope: vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }