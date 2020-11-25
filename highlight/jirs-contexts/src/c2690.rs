
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
  prototype: Some(
    ContextId {
        index: 2664,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:arch|argv|argv0|channel|config|connected|debugPort|env|execArgv|execPath|exitCode|mainModule|noDeprecation|pid|platform|ppid|release|stderr|stdin|stdout|throwDeprecation|title|traceDeprecation|version|versions)(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 61925409751171115,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:abort|chdir|cpuUsage|cwd|disconnect|dlopen|emitWarning|exit|getegid|geteuid|getgit|getgroups|getuid|hasUncaughtExceptionCaptureCallback|hrtime|initGroups|kill|memoryUsage|nextTick|send|setegid|seteuid|setgid|setgroups|setuid|hasUncaughtExceptionCaptureCallback|umask|uptime)(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 61925255132348459,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }