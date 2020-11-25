
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
        a: 281496454758400,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281496454758400,
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
      regex: Regex::new("(?x)^\n(?=  (?:[ ]{,3}>(?:.|(?m:$)))\n|    (?:[ ]{4}|\\t)(?!(?m:$))\n|    (?:[#]{1,6}\\s*)\n|    (?x:\n    [ ]{,3}                          # between 0 to 3 spaces\n    (?:                              # followed by one of the following:\n            [-](?:[ ]{,2}[-]){2,}    # - a dash,        followed by the following at least twice: between 0 to 2 spaces followed by a dash\n        |   [*](?:[ ]{,2}[*]){2,}    # - a star,        followed by the following at least twice: between 0 to 2 spaces followed by a star\n        |   [_](?:[ ]{,2}[_]){2,}    # - an underscore, followed by the following at least twice: between 0 to 2 spaces followed by an underscore\n    )\n    [ \\t]*(?m:$)                          # followed by any number of tabs or spaces, followed by the end of the line\n)\n|    (?x:\n    (?:(?x:\n  (?:\n    (?x:\n  (?:\n    \\\\[-`*_#+.!(){}\\[\\]\\\\>|~]+                     # escape characters\n  | [^\\[\\]`\\\\_*|]+(?=[\\[\\]`\\\\_*|]|(?m:$))  # anything that isn\'t a square bracket, a backtick, the start of an escape character, or an emphasis character\n  | (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)                   # inline code\n  | \\[(?:                           # nested square brackets (one level deep)\n        [^\\[\\]`]+(?=[\\[\\]`])        #  anything that isn\'t a square bracket or a backtick\n        (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)?              #  balanced backticks\n      )*\\]                          #  closing square bracket\n  )+                                # at least one character\n)\n  | (?x:\n    \\*  (?!\\*)(?x:\n  (?:\n    \\\\[-`*_#+.!(){}\\[\\]\\\\>|~]+                     # escape characters\n  | [^\\[\\]`\\\\_*]+(?=[\\[\\]`\\\\_*]|(?m:$))  # anything that isn\'t a square bracket, a backtick, the start of an escape character, or an emphasis character\n  | (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)                   # inline code\n  | \\[(?:                           # nested square brackets (one level deep)\n        [^\\[\\]`]+(?=[\\[\\]`])        #  anything that isn\'t a square bracket or a backtick\n        (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)?              #  balanced backticks\n      )*\\]                          #  closing square bracket\n  )+                                # at least one character\n)+\\*  (?!\\*)\n|   \\*\\*      (?x:\n  (?:\n    \\\\[-`*_#+.!(){}\\[\\]\\\\>|~]+                     # escape characters\n  | [^\\[\\]`\\\\_*]+(?=[\\[\\]`\\\\_*]|(?m:$))  # anything that isn\'t a square bracket, a backtick, the start of an escape character, or an emphasis character\n  | (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)                   # inline code\n  | \\[(?:                           # nested square brackets (one level deep)\n        [^\\[\\]`]+(?=[\\[\\]`])        #  anything that isn\'t a square bracket or a backtick\n        (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)?              #  balanced backticks\n      )*\\]                          #  closing square bracket\n  )+                                # at least one character\n)+\\*\\*\n|   _   (?!_) (?x:\n  (?:\n    \\\\[-`*_#+.!(){}\\[\\]\\\\>|~]+                     # escape characters\n  | [^\\[\\]`\\\\_*]+(?=[\\[\\]`\\\\_*]|(?m:$))  # anything that isn\'t a square bracket, a backtick, the start of an escape character, or an emphasis character\n  | (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)                   # inline code\n  | \\[(?:                           # nested square brackets (one level deep)\n        [^\\[\\]`]+(?=[\\[\\]`])        #  anything that isn\'t a square bracket or a backtick\n        (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)?              #  balanced backticks\n      )*\\]                          #  closing square bracket\n  )+                                # at least one character\n)+_   (?!_)\n|   __        (?x:\n  (?:\n    \\\\[-`*_#+.!(){}\\[\\]\\\\>|~]+                     # escape characters\n  | [^\\[\\]`\\\\_*]+(?=[\\[\\]`\\\\_*]|(?m:$))  # anything that isn\'t a square bracket, a backtick, the start of an escape character, or an emphasis character\n  | (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)                   # inline code\n  | \\[(?:                           # nested square brackets (one level deep)\n        [^\\[\\]`]+(?=[\\[\\]`])        #  anything that isn\'t a square bracket or a backtick\n        (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)?              #  balanced backticks\n      )*\\]                          #  closing square bracket\n  )+                                # at least one character\n)+__\n)\n  )+                                # at least one character\n)?\\|){2}           # at least 2 non-escaped pipe chars on the line\n|   (?!\\s+\\|)(?x:\n  (?:\n    (?x:\n  (?:\n    \\\\[-`*_#+.!(){}\\[\\]\\\\>|~]+                     # escape characters\n  | [^\\[\\]`\\\\_*|]+(?=[\\[\\]`\\\\_*|]|(?m:$))  # anything that isn\'t a square bracket, a backtick, the start of an escape character, or an emphasis character\n  | (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)                   # inline code\n  | \\[(?:                           # nested square brackets (one level deep)\n        [^\\[\\]`]+(?=[\\[\\]`])        #  anything that isn\'t a square bracket or a backtick\n        (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)?              #  balanced backticks\n      )*\\]                          #  closing square bracket\n  )+                                # at least one character\n)\n  | (?x:\n    \\*  (?!\\*)(?x:\n  (?:\n    \\\\[-`*_#+.!(){}\\[\\]\\\\>|~]+                     # escape characters\n  | [^\\[\\]`\\\\_*]+(?=[\\[\\]`\\\\_*]|(?m:$))  # anything that isn\'t a square bracket, a backtick, the start of an escape character, or an emphasis character\n  | (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)                   # inline code\n  | \\[(?:                           # nested square brackets (one level deep)\n        [^\\[\\]`]+(?=[\\[\\]`])        #  anything that isn\'t a square bracket or a backtick\n        (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)?              #  balanced backticks\n      )*\\]                          #  closing square bracket\n  )+                                # at least one character\n)+\\*  (?!\\*)\n|   \\*\\*      (?x:\n  (?:\n    \\\\[-`*_#+.!(){}\\[\\]\\\\>|~]+                     # escape characters\n  | [^\\[\\]`\\\\_*]+(?=[\\[\\]`\\\\_*]|(?m:$))  # anything that isn\'t a square bracket, a backtick, the start of an escape character, or an emphasis character\n  | (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)                   # inline code\n  | \\[(?:                           # nested square brackets (one level deep)\n        [^\\[\\]`]+(?=[\\[\\]`])        #  anything that isn\'t a square bracket or a backtick\n        (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)?              #  balanced backticks\n      )*\\]                          #  closing square bracket\n  )+                                # at least one character\n)+\\*\\*\n|   _   (?!_) (?x:\n  (?:\n    \\\\[-`*_#+.!(){}\\[\\]\\\\>|~]+                     # escape characters\n  | [^\\[\\]`\\\\_*]+(?=[\\[\\]`\\\\_*]|(?m:$))  # anything that isn\'t a square bracket, a backtick, the start of an escape character, or an emphasis character\n  | (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)                   # inline code\n  | \\[(?:                           # nested square brackets (one level deep)\n        [^\\[\\]`]+(?=[\\[\\]`])        #  anything that isn\'t a square bracket or a backtick\n        (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)?              #  balanced backticks\n      )*\\]                          #  closing square bracket\n  )+                                # at least one character\n)+_   (?!_)\n|   __        (?x:\n  (?:\n    \\\\[-`*_#+.!(){}\\[\\]\\\\>|~]+                     # escape characters\n  | [^\\[\\]`\\\\_*]+(?=[\\[\\]`\\\\_*]|(?m:$))  # anything that isn\'t a square bracket, a backtick, the start of an escape character, or an emphasis character\n  | (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)                   # inline code\n  | \\[(?:                           # nested square brackets (one level deep)\n        [^\\[\\]`]+(?=[\\[\\]`])        #  anything that isn\'t a square bracket or a backtick\n        (?x:\n    (`{4})(?![\\s`])(?:[^`]+(?=`)|(?!`{4})`+(?!`))+(`{4})(?!`)  # 4 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 4 backticks, or at least one non backtick character) at least once, followed by exactly 4 backticks\n|   (`{3})(?![\\s`])(?:[^`]+(?=`)|(?!`{3})`+(?!`))+(`{3})(?!`)  # 3 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 3 backticks, or at least one non backtick character) at least once, followed by exactly 3 backticks\n|   (`{2})(?![\\s`])(?:[^`]+(?=`)|(?!`{2})`+(?!`))+(`{2})(?!`)  # 2 backticks, followed by at least one non whitespace, non backtick character, followed by (less than 2 backticks, or at least one non backtick character) at least once, followed by exactly 2 backticks\n|   (`{1})(?![\\s`])(?:[^`]+(?=`)|(?!`{1})`+(?!`))+(`{1})(?!`)  # 1 backtick,  followed by at least one non whitespace, non backtick character, followed by (                          at least one non backtick character) at least once, followed by exactly 1 backtick\n)?              #  balanced backticks\n      )*\\]                          #  closing square bracket\n  )+                                # at least one character\n)+__\n)\n  )+                                # at least one character\n)\\|(?!\\s+(?m:$))  # something other than whitespace followed by a pipe char, followed by something other than whitespace and the end of the line\n)\n)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3286 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^([ ]{0,3})([*+-])(?=\\s)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 114280017427103793,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 114280017427104681,
            b: 13792273858822144,
        },
        Scope {
            a: 47288629372518449,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3287 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^([ ]{0,3})(\\d+(\\.))(?=\\s)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 114280017426907185,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 114280017426908073,
            b: 13792273858822144,
        },
    ]),(3, vec![
        Scope {
            a: 47288629372518449,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3288 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^[ ]{0,3}(?=<((?i:pre))\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3322 }),
        ContextReference::Direct(ContextId { index: 3321 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^[ ]{0,3}(?=<(?xi:\n  (script|style|pre)\\b\n))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3322 }),
        ContextReference::Direct(ContextId { index: 3318 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^[ ]{0,3}(?=<\\?)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3322 }),
        ContextReference::Direct(ContextId { index: 3320 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^[ ]{0,3}(?=<!(?:[A-Z]|--))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3322 }),
        ContextReference::Direct(ContextId { index: 3317 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^[ ]{0,3}(?=<!\\[CDATA\\[)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3322 }),
        ContextReference::Direct(ContextId { index: 3316 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^[ ]{0,3}(?=(?:(?xi:\n  <\n  [a-z]             # A tag name consists of an ASCII letter\n  [a-z0-9-]*        # followed by zero or more ASCII letters, digits, or hyphens (-)\n  (?:               # An attribute consists of whitespace, an attribute name, and an optional attribute value specification\n    \\s+\n    [a-z_:]         # An attribute name consists of an ASCII letter, _, or :\n    [a-z0-9_.:-]*   # followed by zero or more ASCII letters, digits, _, ., :, or -\n    (?:             # An attribute value specification consists of optional whitespace, a = character, optional whitespace, and an attribute value\n      \\s*\n      =\n      \\s*\n      (?:\n        [^ @\'=<>`]+ # An unquoted attribute value is a nonempty string of characters not including spaces, \", \', =, <, >, or `\n      | \'[^\']*\'     # A single-quoted attribute value consists of \', zero or more characters not including \', and a final \'\n      | \"[^\"]*\"     # A double-quoted attribute value consists of \", zero or more characters not including \", and a final \"\n      )\n    )?\n  )*\n  \\s*\n  /?\n  >\n)|(?xi:\n  </\n  [a-z]             # A tag name consists of an ASCII letter\n  [a-z0-9-]*        # followed by zero or more ASCII letters, digits, or hyphens (-)\n  \\s*\n  >\n))\\s*(?m:$)|<(?x:\n  /?\n  (?i:address|article|aside|base|basefont|blockquote|body|caption|center|col|colgroup|dd|details|dialog|dir|div|dl|dt|fieldset|figcaption|figure|footer|form|frame|frameset|h1|h2|h3|h4|h5|h6|head|header|hr|html|iframe|legend|li|link|main|menu|menuitem|meta|nav|noframes|ol|optgroup|option|p|param|section|source|summary|table|tbody|td|tfoot|th|thead|title|tr|track|ul)\n  (?:\\s|(?m:$)|/?>)\n))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3322 }),
        ContextReference::Direct(ContextId { index: 3319 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n    \\s*                        # Leading whitespace\n    (\\[)(\\^[^]]*)(\\])(:)       # Reference name\n    [ \\t]*                     # Optional whitespace\n)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325004982,
            b: 13792273858822144,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630418459,
            b: 13792273858822144,
        },
    ]),(3, vec![
        Scope {
            a: 47288629325004971,
            b: 13792273858822144,
        },
    ]),(4, vec![
        Scope {
            a: 47288620737429553,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3289 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n    \\s*                        # Leading whitespace\n    (\\[)([^]]*)(\\])(:)         # Reference name\n    [ \\t]*                     # Optional whitespace\n    (?:\n      (<)([^>]+)(>)            # The url\n    | (\\S+)                    # The url\n    )\n)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325004982,
            b: 13792273858822144,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630418459,
            b: 13792273858822144,
        },
    ]),(3, vec![
        Scope {
            a: 47288629325004971,
            b: 13792273858822144,
        },
    ]),(4, vec![
        Scope {
            a: 47288620737429553,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629312815286,
            b: 13792273858822144,
        },
    ]),(6, vec![
        Scope {
            a: 114280588597985329,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 47288629312815275,
            b: 13792273858822144,
        },
    ]),(8, vec![
        Scope {
            a: 114280588597985329,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3349 }),
        ContextReference::Direct(ContextId { index: 3304 }),
        ContextReference::Direct(ContextId { index: 3358 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(?=\\S)(?![=-]{3,}\\s*(?m:$))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3290 }),
    ]),
      with_prototype: None
    }),
]
} }