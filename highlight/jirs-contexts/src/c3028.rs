
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
      regex: Regex::new("\\b(?i:with|while|when|unless|typecase|to|thereis|then|return-from name|return|repeat|prog*|prog|never|named|maplist|mapl|mapcon|mapcar|mapcan|mapc|loop|let|initially|if|from|for|finally|etypecase|else|dotimes|dolist|doing|do*|do|ctypecase|cond|case|block|as|always)\\b"),
      scope: vec![
        Scope {
            a: 52636636691693568,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }