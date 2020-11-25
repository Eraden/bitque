
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
      regex: Regex::new("(?<=\\))(?!(([_$\\p{L}][_$\\p{L}\\p{N}]*\\s*\\??\\.\\s*)*|(\\??\\.\\s*)?)([_$\\p{L}][_$\\p{L}\\p{N}]*)\\s*(\\?\\.\\s*)?(<\\s*(((keyof|infer)\\s+)|(([_$\\p{L}][_$\\p{L}\\p{N}]*|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])|(\\\'[^\\\']*\\\')|(\\\"[^\\\"]*\\\")|(\\`[^\\`]*\\`))(?=\\s*([\\<\\>\\,\\.\\[=]|&(?!&)|\\|(?!\\|)))))([^<>\\(]|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(?<==)\\>|\\<\\s*(((keyof|infer)\\s+)|(([_$\\p{L}][_$\\p{L}\\p{N}]*|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])|(\\\'[^\\\']*\\\')|(\\\"[^\\\"]*\\\")|(\\`[^\\`]*\\`))(?=\\s*([\\<\\>\\,\\.\\[=]|&(?!&)|\\|(?!\\|)))))([^<>\\(]|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(?<==)\\>)*(?!=)\\>)*(?!=)>\\s*)?\\()"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(([_$\\p{L}][_$\\p{L}\\p{N}]*\\s*\\??\\.\\s*)*|(\\??\\.\\s*)?)([_$\\p{L}][_$\\p{L}\\p{N}]*))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9746 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9873 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\?\\."),
      scope: vec![
        Scope {
            a: 46444882995642368,
            b: 0,
        },
        Scope {
            a: 47288788293582999,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9980 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9952 })),
]
} }