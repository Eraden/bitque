
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
      regex: Regex::new("\\b(?i:true|false|yes|no)\\b"),
      scope: vec![
        Scope {
            a: 59955110657196040,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:null|missing\\s+value)\\b"),
      scope: vec![
        Scope {
            a: 59955110657261576,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("-?\\b\\d+((\\.(\\d+\\b)?)?(?i:e\\+?\\d*\\b)?|\\b)"),
      scope: vec![
        Scope {
            a: 59955089162895360,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:space|tab|return|linefeed|quote)\\b"),
      scope: vec![
        Scope {
            a: 59955136407076872,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:all\\s+(caps|lowercase)|bold|condensed|expanded|hidden|italic|outline|plain|shadow|small\\s+caps|strikethrough|(sub|super)script|underline)\\b"),
      scope: vec![
        Scope {
            a: 59955136427130888,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:Jan(uary)?|Feb(ruary)?|Mar(ch)?|Apr(il)?|May|Jun(e)?|Jul(y)?|Aug(ust)?|Sep(tember)?|Oct(ober)?|Nov(ember)?|Dec(ember)?)\\b"),
      scope: vec![
        Scope {
            a: 59955136427196725,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:Mon(day)?|Tue(sday)?|Wed(nesday)?|Thu(rsday)?|Fri(day)?|Sat(urday)?|Sun(day)?)\\b"),
      scope: vec![
        Scope {
            a: 59955136427196726,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:AppleScript|pi|result|version|current\\s+application|its?|m[ey])\\b"),
      scope: vec![
        Scope {
            a: 59955136427393032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:text\\s+item\\s+delimiters|print\\s+(length|depth))\\b"),
      scope: vec![
        Scope {
            a: 49259061522726912,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }