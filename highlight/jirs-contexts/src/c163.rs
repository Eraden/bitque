
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
      regex: Regex::new("\\b((alert|dialog) reply)\\b"),
      scope: vec![
        Scope {
            a: 61925366776136008,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(file information)\\b"),
      scope: vec![
        Scope {
            a: 61925366776136009,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(POSIX files?|system information|volume settings)\\b"),
      scope: vec![
        Scope {
            a: 61925366776135991,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(URLs?|internet address(es)?|web pages?|FTP items?)\\b"),
      scope: vec![
        Scope {
            a: 61925366776136010,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(info for|list (disks|folder)|mount volume|path to( resource)?)\\b"),
      scope: vec![
        Scope {
            a: 61925255106986313,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(beep|choose (application|color|file( name)?|folder|from list|remote application|URL)|delay|display (alert|dialog)|say)\\b"),
      scope: vec![
        Scope {
            a: 61925255106986312,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(ASCII (character|number)|localized string|offset|summarize)\\b"),
      scope: vec![
        Scope {
            a: 61925255106986181,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(set the clipboard to|the clipboard|clipboard info)\\b"),
      scope: vec![
        Scope {
            a: 61925255106986315,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(open for access|close access|read|write|get eof|set eof)\\b"),
      scope: vec![
        Scope {
            a: 61925255106986316,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((load|store|run) script|scripting components)\\b"),
      scope: vec![
        Scope {
            a: 61925255106986317,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(current date|do shell script|get volume settings|random number|round|set volume|system attribute|system info|time to GMT)\\b"),
      scope: vec![
        Scope {
            a: 61925255106986295,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(opening folder|(closing|moving) folder window for|adding folder items to|removing folder items from)\\b"),
      scope: vec![
        Scope {
            a: 61925255106986318,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(open location|handle CGI request)\\b"),
      scope: vec![
        Scope {
            a: 61925255106986314,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }