
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
      regex: Regex::new("(?x)(?=\\}|;|,|(?m:$)|(^(?!\\s*((\\b(?<!\\$)0(x|X)[0-9a-fA-F][0-9a-fA-F_]*\\b(?!\\$))|(\\b(?<!\\$)0(b|B)[01][01_]*\\b(?!\\$))|(\\b(?<!\\$)0(o|O)?[0-7][0-7_]*\\b(?!\\$))|((?<!\\$)(?:\n  (?:\\b[0-9][0-9_]*(\\.)[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*\\b)| # 1.1E+3\n  (?:\\b[0-9][0-9_]*(\\.)[eE][+-]?[0-9][0-9_]*\\b)|             # 1.E+3\n  (?:\\B(\\.)[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*\\b)|             # .1E+3\n  (?:\\b[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*\\b)|                 # 1E+3\n  (?:\\b[0-9][0-9_]*(\\.)[0-9][0-9_]*\\b)|                      # 1.1\n  (?:\\b[0-9][0-9_]*(\\.)\\B)|                                  # 1.\n  (?:\\B(\\.)[0-9][0-9_]*\\b)|                                  # .1\n  (?:\\b[0-9][0-9_]*\\b(?!\\.))                                 # 1\n)(?!\\$))|([_$\\p{L}][_$\\p{L}\\p{N}]*)|(\\\'([^\\\'\\\\]|\\\\\\\'|\\\\)*\\\')|(\\\"([^\\\"\\\\]|\\\\\\\"|\\\\)*\\\")|(\\[([^\\[\\]]|\\[[^\\[\\]]*\\])+\\]))\\s*(\\?\\s*)?(=|:))))|(?<=\\})"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10000 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(?=((\\b(?<!\\$)0(x|X)[0-9a-fA-F][0-9a-fA-F_]*\\b(?!\\$))|(\\b(?<!\\$)0(b|B)[01][01_]*\\b(?!\\$))|(\\b(?<!\\$)0(o|O)?[0-7][0-7_]*\\b(?!\\$))|((?<!\\$)(?:\n  (?:\\b[0-9][0-9_]*(\\.)[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*\\b)| # 1.1E+3\n  (?:\\b[0-9][0-9_]*(\\.)[eE][+-]?[0-9][0-9_]*\\b)|             # 1.E+3\n  (?:\\B(\\.)[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*\\b)|             # .1E+3\n  (?:\\b[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*\\b)|                 # 1E+3\n  (?:\\b[0-9][0-9_]*(\\.)[0-9][0-9_]*\\b)|                      # 1.1\n  (?:\\b[0-9][0-9_]*(\\.)\\B)|                                  # 1.\n  (?:\\B(\\.)[0-9][0-9_]*\\b)|                                  # .1\n  (?:\\b[0-9][0-9_]*\\b(?!\\.))                                 # 1\n)(?!\\$))|([_$\\p{L}][_$\\p{L}\\p{N}]*)|(\\\'([^\\\'\\\\]|\\\\\\\'|\\\\)*\\\')|(\\\"([^\\\"\\\\]|\\\\\\\"|\\\\)*\\\")|(\\[([^\\[\\]]|\\[[^\\[\\]]*\\])+\\]))\\s*(\\?\\s*)?(=|:))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9742 }),
    ]),
      with_prototype: None
    }),
]
} }