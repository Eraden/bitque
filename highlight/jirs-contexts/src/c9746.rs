
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
      regex: Regex::new("(?=\\s*(\\?\\.\\s*)?(<\\s*(((keyof|infer)\\s+)|(([_$\\p{L}][_$\\p{L}\\p{N}]*|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])|(\\\'[^\\\']*\\\')|(\\\"[^\\\"]*\\\")|(\\`[^\\`]*\\`))(?=\\s*([\\<\\>\\,\\.\\[=]|&(?!&)|\\|(?!\\|)))))([^<>\\(]|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(?<==)\\>|\\<\\s*(((keyof|infer)\\s+)|(([_$\\p{L}][_$\\p{L}\\p{N}]*|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])|(\\\'[^\\\']*\\\')|(\\\"[^\\\"]*\\\")|(\\`[^\\`]*\\`))(?=\\s*([\\<\\>\\,\\.\\[=]|&(?!&)|\\|(?!\\|)))))([^<>\\(]|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(?<==)\\>)*(?!=)\\>)*(?!=)>\\s*)?\\()"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9929 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9971 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9943 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9957 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))import(?=\\s*[\\(]\\s*[\\\"\\\'\\`]))"),
      scope: vec![
        Scope {
            a: 52636628121289143,
            b: 42502721483309056,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([_$\\p{L}][_$\\p{L}\\p{N}]*)"),
      scope: vec![
        Scope {
            a: 59392130630615191,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }