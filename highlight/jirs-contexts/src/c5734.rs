
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
      regex: Regex::new("(\\{)(\\*)(\\})"),
      scope: vec![
        Scope {
            a: 46444328940077056,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521951477942,
            b: 21955048183431168,
        },
    ]),(2, vec![
        Scope {
            a: 52636628103856128,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288521951477931,
            b: 21955048183431168,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{(?=\\s*(while|for|catch|return|break|continue|switch|exit|foreach|if|after|append|array|auto_execok|auto_import|auto_load|auto_mkindex|auto_mkindex_old|auto_qualify|auto_reset|bgerror|binary|cd|clock|close|concat|dde|encoding|eof|error|eval|exec|expr|fblocked|fconfigure|fcopy|file|fileevent|filename|flush|format|gets|glob|global|history|http|incr|info|interp|join|lappend|library|lindex|linsert|list|llength|load|lrange|lreplace|lsearch|lset|lsort|memory|msgcat|namespace|open|package|parray|pid|pkg::create|pkg_mkIndex|proc|puts|pwd|re_syntax|read|registry|rename|resource|scan|seek|set|socket|SafeBase|source|split|string|subst|Tcl|tcl_endOfWord|tcl_findLibrary|tcl_startOfNextWord|tcl_startOfPreviousWord|tcl_wordBreakAfter|tcl_wordBreakBefore|tcltest|tclvars|tell|time|trace|unknown|unset|update|uplevel|upvar|variable|vwait)\\b|\\n)"),
      scope: vec![
        Scope {
            a: 47288521951477942,
            b: 21955048183431168,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5703 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{(?=\\s|\\})"),
      scope: vec![
        Scope {
            a: 47288521951477942,
            b: 21955048183431168,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5748 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{"),
      scope: vec![
        Scope {
            a: 47288629323956406,
            b: 21955048183431168,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5756 }),
    ]),
      with_prototype: None
    }),
]
} }