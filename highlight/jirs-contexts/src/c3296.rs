
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 46446613918777344,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46446613918777344,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)^\n(?=  (?:[ ]{,3}>(?:.|(?m:$)))\n|    (?:[ ]{4}|\\t)(?!(?m:$))\n|    (?:[#]{1,6}\\s*)\n|    (?x:\n    [ ]{,3}                          # between 0 to 3 spaces\n    (?:                              # followed by one of the following:\n            [-](?:[ ]{,2}[-]){2,}    # - a dash,        followed by the following at least twice: between 0 to 2 spaces followed by a dash\n        |   [*](?:[ ]{,2}[*]){2,}    # - a star,        followed by the following at least twice: between 0 to 2 spaces followed by a star\n        |   [_](?:[ ]{,2}[_]){2,}    # - an underscore, followed by the following at least twice: between 0 to 2 spaces followed by an underscore\n    )\n    [ \\t]*(?m:$)                          # followed by any number of tabs or spaces, followed by the end of the line\n)\n|    \\s*(?m:$)\n)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\|"),
      scope: vec![
        Scope {
            a: 47288620783763505,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(?x:\n    \\*  (?!\\*)(?x:\n  (?:\n    \\\\[-`*_#+.!(){}\\[\\]\\\\>|~]+                     # escape characters\n  | [^\\[\\]`\\\\_*]+(?=[\\[\\]`\\\\_*]|(?m:$))  # anything that isn\'t a square bracket, a backtick, the start of an escape character, or an emphasis character\n  | (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)                   # inline code\n  | \\[(?:                           # nested square brackets (one level deep)\n        [^\\[\\]`]+(?=[\\[\\]`])        #  anything that isn\'t a square bracket or a backtick\n        (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)?              #  balanced backticks\n      )*\\]                          #  closing square bracket\n  )+                                # at least one character\n)+\\*  (?!\\*)\n|   \\*\\*      (?x:\n  (?:\n    \\\\[-`*_#+.!(){}\\[\\]\\\\>|~]+                     # escape characters\n  | [^\\[\\]`\\\\_*]+(?=[\\[\\]`\\\\_*]|(?m:$))  # anything that isn\'t a square bracket, a backtick, the start of an escape character, or an emphasis character\n  | (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)                   # inline code\n  | \\[(?:                           # nested square brackets (one level deep)\n        [^\\[\\]`]+(?=[\\[\\]`])        #  anything that isn\'t a square bracket or a backtick\n        (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)?              #  balanced backticks\n      )*\\]                          #  closing square bracket\n  )+                                # at least one character\n)+\\*\\*\n|   _   (?!_) (?x:\n  (?:\n    \\\\[-`*_#+.!(){}\\[\\]\\\\>|~]+                     # escape characters\n  | [^\\[\\]`\\\\_*]+(?=[\\[\\]`\\\\_*]|(?m:$))  # anything that isn\'t a square bracket, a backtick, the start of an escape character, or an emphasis character\n  | (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)                   # inline code\n  | \\[(?:                           # nested square brackets (one level deep)\n        [^\\[\\]`]+(?=[\\[\\]`])        #  anything that isn\'t a square bracket or a backtick\n        (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)?              #  balanced backticks\n      )*\\]                          #  closing square bracket\n  )+                                # at least one character\n)+_   (?!_)\n|   __        (?x:\n  (?:\n    \\\\[-`*_#+.!(){}\\[\\]\\\\>|~]+                     # escape characters\n  | [^\\[\\]`\\\\_*]+(?=[\\[\\]`\\\\_*]|(?m:$))  # anything that isn\'t a square bracket, a backtick, the start of an escape character, or an emphasis character\n  | (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)                   # inline code\n  | \\[(?:                           # nested square brackets (one level deep)\n        [^\\[\\]`]+(?=[\\[\\]`])        #  anything that isn\'t a square bracket or a backtick\n        (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)?              #  balanced backticks\n      )*\\]                          #  closing square bracket\n  )+                                # at least one character\n)+__\n))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3297 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(?!(?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n))\n`+"),
      scope: vec![
        Scope {
            a: 50104723465830449,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3337 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2107 })),
]
} }