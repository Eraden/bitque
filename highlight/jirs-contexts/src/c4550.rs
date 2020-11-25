
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
      regex: Regex::new("(?x)\\b(\n\t(\n\t\tArithmetic|Assertion|Attribute|BlockingIO|BrokenPipe|Buffer|ChildProcess|\n\t\tConnection(Aborted|Refused|Reset)?|EOF|Environment|FileExists|\n\t\tFileNotFound|FloatingPoint|Interrupted|IO|IsADirectoryError|\n\t\tImport|Indentation|Index|Key|Lookup|Memory|Name|NotADirectory|\n\t\tNotImplemented|OS|Overflow|Permission|ProcessLookup|Reference|\n\t\tRuntime|Standard|Syntax|System|Tab|Timeout|Type|UnboundLocal|\n\t\tUnicode(Encode|Decode|Translate)?|Value|VMS|Windows|ZeroDivision\n\t)Error|\n\t((Pending)?Deprecation|Runtime|Syntax|User|Future|Import|Unicode|Bytes)?Warning|\n\t(Base)?Exception|\n\tSystemExit|StopIteration|NotImplemented|KeyboardInterrupt|GeneratorExit\n)\\b"),
      scope: vec![
        Scope {
            a: 61925375362138174,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }