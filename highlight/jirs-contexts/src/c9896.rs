
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
      regex: Regex::new("(?=(([_$\\p{L}][_$\\p{L}\\p{N}]*\\s*\\??\\.\\s*)*|(\\??\\.\\s*)?)([_$\\p{L}][_$\\p{L}\\p{N}]*)\\s*(\\?\\.\\s*)?(<\\s*(((keyof|infer)\\s+)|(([_$\\p{L}][_$\\p{L}\\p{N}]*|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])|(\\\'[^\\\']*\\\')|(\\\"[^\\\"]*\\\")|(\\`[^\\`]*\\`))(?=\\s*([\\<\\>\\,\\.\\[=]|&(?!&)|\\|(?!\\|)))))([^<>\\(]|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(?<==)\\>|\\<\\s*(((keyof|infer)\\s+)|(([_$\\p{L}][_$\\p{L}\\p{N}]*|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])|(\\\'[^\\\']*\\\')|(\\\"[^\\\"]*\\\")|(\\`[^\\`]*\\`))(?=\\s*([\\<\\>\\,\\.\\[=]|&(?!&)|\\|(?!\\|)))))([^<>\\(]|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(?<==)\\>)*(?!=)\\>)*(?!=)>\\s*)?\\()"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9745 }),
    ]),
      with_prototype: None
    }),
]
} }