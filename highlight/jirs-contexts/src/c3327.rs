
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
      regex: Regex::new("(?x:\n    (\\!\\[)                             # Images start with ![\n    (?=   (?x:\n  (?:\n    \\\\[-`*_#+.!(){}\\[\\]\\\\>|~]+                 # escape characters\n  | [^\\[\\]`\\\\]+(?=[\\[\\]`\\\\]|$)  # anything that isn\'t a square bracket or a backtick or the start of an escape character\n  | (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)               # inline code\n  | \\[(?:                       # nested square brackets (one level deep)\n        [^\\[\\]`]+(?=[\\[\\]`])    #  anything that isn\'t a square bracket or a backtick\n        (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)?          #  balanced backticks\n      )*\\]                      #  closing square bracket\n  )+                            # at least one character\n)? # balanced square brackets, backticks, taking into account escapes etc.\n          \\]                           # Closing square bracket\n          [ ]?                         # Space not allowed, but we check for it anyway to mark it as invalid\n          \\(                           # Open paren\n    )\n )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445673309077553,
            b: 0,
        },
        Scope {
            a: 47288629346173110,
            b: 13792273858822144,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3329 }),
        ContextReference::Direct(ContextId { index: 3328 }),
        ContextReference::Direct(ContextId { index: 3330 }),
    ]),
      with_prototype: None
    }),
]
} }