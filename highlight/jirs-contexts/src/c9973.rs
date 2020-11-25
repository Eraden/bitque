
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
      regex: Regex::new("([_$\\p{L}][_$\\p{L}\\p{N}]*)\\s*(?=(<\\s*(((keyof|infer)\\s+)|(([_$\\p{L}][_$\\p{L}\\p{N}]*|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])|(\\\'[^\\\']*\\\')|(\\\"[^\\\"]*\\\")|(\\`[^\\`]*\\`))(?=\\s*([\\<\\>\\,\\.\\[=]|&(?!&)|\\|(?!\\|)))))([^<>\\(]|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(?<==)\\>|\\<\\s*(((keyof|infer)\\s+)|(([_$\\p{L}][_$\\p{L}\\p{N}]*|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])|(\\\'[^\\\']*\\\')|(\\\"[^\\\"]*\\\")|(\\`[^\\`]*\\`))(?=\\s*([\\<\\>\\,\\.\\[=]|&(?!&)|\\|(?!\\|)))))([^<>\\(]|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(?<==)\\>)*(?!=)\\>)*(?!=)>\\s*)`)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130630615874,
            b: 42502721483309056,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9819 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([_$\\p{L}][_$\\p{L}\\p{N}]*)?(`)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130630615874,
            b: 42502721483309056,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956666,
            b: 51229094301401088,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9820 }),
    ]),
      with_prototype: None
    }),
]
} }