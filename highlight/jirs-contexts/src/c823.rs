
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
      regex: Regex::new("(?x)%\n  (\\d+\\$)?                                      # field (argument #)\n  [#0\\- +\']*                                    # flags\n  [,;:_]?                                       # separator character (AltiVec)\n  ((-?\\d+)|\\*(-?\\d+\\$)?)?                       # minimum field width\n  (\\.((-?\\d+)|\\*(-?\\d+\\$)?)?)?                  # precision\n  (hh|h|ll|l|j|t|z|q|L|vh|vl|v|hv|hl)?          # length modifier\n  (\\[[^\\]]+\\]|[am]s|[diouxXDOUeEfFgGaACcSspn%]) # conversion type"),
      scope: vec![
        Scope {
            a: 59955136434012173,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }