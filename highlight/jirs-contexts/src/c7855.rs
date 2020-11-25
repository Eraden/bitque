
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
      regex: Regex::new("(?<!\\.)\\b(constructor|decodeURI|decodeURIComponent|encodeURI|encodeURIComponent|escape|eval|hasOwnProperty|isFinite|isNaN|isPrototypeOf|parseFloat|parseInt|propertyIsEnumerable|toLocaleString|toString|unescape|valueOf)\\b"),
      scope: vec![
        Scope {
            a: 61925255090602027,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(clearImmediate|clearInterval|clearTimeout|require|setImmediate|setInterval|setTimeout)\\b"),
      scope: vec![
        Scope {
            a: 61925255132348459,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(?>document|window)\\b"),
      scope: vec![
        Scope {
            a: 61925375399035713,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(?>global|GLOBAL|root|__dirname|__filename)\\b"),
      scope: vec![
        Scope {
            a: 61925375399035594,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(console)(?:(\\.)(assert|count|dir|error|group|groupCollapsed|groupEnd|info|log|profile|profileEnd|table|time|timeEnd|trace|warn))?\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925375399035701,
            b: 12103423998558208,
        },
    ]),(2, vec![
        Scope {
            a: 52636628113883179,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925255139360811,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(process)(?:(\\.)(?:(arch|argv|config|env|execArgv|execPath|exitCode|mainModule|pid|platform|stderr|stdin|stdout|title|version|versions)|(abort|chdir|cwd|exit|getgid|getgroups|getuid|hrtime|initgroups|kill|memoryUsage|nextTick|setgid|setgroups|setuid|umask|uptime)))?\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925375399036321,
            b: 12103423998558208,
        },
    ]),(2, vec![
        Scope {
            a: 52636628113883179,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925375399036321,
            b: 12103423998558208,
        },
    ]),(4, vec![
        Scope {
            a: 61925255179993131,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(exports|module)(?:(\\.)(children|exports|filename|id|loaded|parent))?\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925375399035643,
            b: 12103423998558208,
        },
    ]),(2, vec![
        Scope {
            a: 52636628113883179,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925375399035643,
            b: 12103423998558208,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("{{"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7794 }),
    ]),
      with_prototype: None
    }),
]
} }