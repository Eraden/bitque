
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
      regex: Regex::new("(?=\\))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9099 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9123 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((?xi:\n    arabic-indic | armenian | bengali | cambodian | circle\n  | cjk-decimal | cjk-earthly-branch | cjk-heavenly-stem | decimal-leading-zero\n  | decimal | devanagari | disclosure-closed | disclosure-open | disc\n  | ethiopic-numeric | georgian | gujarati | gurmukhi | hebrew\n  | hiragana-iroha | hiragana | japanese-formal | japanese-informal\n  | kannada | katakana-iroha | katakana | khmer\n  | korean-hangul-formal | korean-hanja-formal | korean-hanja-informal | lao\n  | lower-alpha | lower-armenian | lower-greek | lower-latin | lower-roman\n  | malayalam | mongolian | myanmar | oriya | persian\n  | simp-chinese-formal | simp-chinese-informal\n  | square | tamil | telugu | thai | tibetan\n  | trad-chinese-formal | trad-chinese-informal\n  | upper-alpha | upper-armenian | upper-latin | upper-roman\n)|none)\\b"),
      scope: vec![
        Scope {
            a: 61925409737015782,
            b: 3940649673949184,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9148 })),
]
} }