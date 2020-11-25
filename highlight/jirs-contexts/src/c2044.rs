
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
      regex: Regex::new("(?x)\\b(?:sprintf|print(?:f|ln)?)\\b"),
      scope: vec![
        Scope {
            a: 61925255134445601,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\\b(?:shouldFail|fail(?:NotEquals)?|ass(?:ume|ert(?:S(?:cript|ame)|N(?:ot(?:Same|\nNull)|ull)|Contains|T(?:hat|oString|rue)|Inspect|Equals|False|Length|\nArrayEquals)))\\b"),
      scope: vec![
        Scope {
            a: 61925255134511137,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\\b(?:sleep|inspect|dump|use|with)\\b"),
      scope: vec![
        Scope {
            a: 61925255100301345,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }