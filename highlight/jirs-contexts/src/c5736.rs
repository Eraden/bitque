
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
      regex: Regex::new("(?=\\bproc\\b\\s)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5751 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bexpr\\b(?=\\s)"),
      scope: vec![
        Scope {
            a: 52636787017646080,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5744 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(namespace)\\s+eval\\s+((?:\\$\\{[^ \\}]+\\}|[^\\s[;{}\\[\\]\"\\\\]][^\\s$[;{}\\[\\]\"\\\\]]*)+)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787017646080,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130643525710,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5704 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(if)\\b\\s+(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636693790720,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444328940077056,
            b: 0,
        },
        Scope {
            a: 47288521951477942,
            b: 21955048183431168,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5739 }),
        ContextReference::Direct(ContextId { index: 5741 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(if)\\b\\s+(?=\\S)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636693790720,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5739 }),
        ContextReference::Direct(ContextId { index: 5740 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bset\\b"),
      scope: vec![
        Scope {
            a: 52636787017646080,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5705 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(while|for|catch|return|break|continue|switch|exit|foreach)\\b(?=[[;\\s\\}\\]\\\\]])"),
      scope: vec![
        Scope {
            a: 52636636693790720,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5710 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(after|append|array|auto_execok|auto_import|auto_load|auto_mkindex|auto_mkindex_old|auto_qualify|auto_reset|bgerror|binary|cd|clock|close|concat|dde|encoding|eof|error|eval|exec|expr|fblocked|fconfigure|fcopy|file|fileevent|filename|flush|format|gets|glob|global|history|http|incr|info|interp|join|lappend|library|lindex|linsert|list|llength|load|lrange|lreplace|lsearch|lset|lsort|memory|msgcat|namespace|open|package|parray|pid|pkg::create|pkg_mkIndex|proc|puts|pwd|re_syntax|read|registry|rename|resource|scan|seek|set|socket|SafeBase|source|split|string|subst|Tcl|tcl_endOfWord|tcl_findLibrary|tcl_startOfNextWord|tcl_startOfPreviousWord|tcl_wordBreakAfter|tcl_wordBreakBefore|tcltest|tclvars|tell|time|trace|unknown|unset|update|uplevel|upvar|variable|vwait)\\b"),
      scope: vec![
        Scope {
            a: 52636787017646080,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(regexp|regsub)\\b(?=\\s)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787017646080,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5711 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=#)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5738 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(?:\\$\\{[^ \\}]+\\}|[^\\s[;{}\\[\\]\"\\\\]][^\\s$[;{}\\[\\]\"\\\\]]*)+)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5707 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }