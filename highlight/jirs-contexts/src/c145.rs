
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
      regex: Regex::new("^\\s*(script)\\s+(\\w+)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636707684360,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632450348,
            b: 2251799813685248,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 132 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(?x)\n\t\\s*(to|on)\\s+ \t\t\t\t\t# \"on\" or \"to\"\n\t(\\w+)\t\t\t\t\t\t\t# function name\n\t(\\()\t\t\t\t\t\t\t# opening paren\n\t\t((?:[\\s,:\\{\\}]*(?:\\w+)?)*)\t# parameters\n\t(\\))\t\t\t\t\t\t\t# closing paren"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636700278792,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615341,
            b: 2251799813685248,
        },
    ]),(3, vec![
        Scope {
            a: 47288629327560712,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 49258876858335240,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629327560712,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 133 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(?x)\n\t\\s*(to|on)\\s+ \t\t\t\t\t# \"on\" or \"to\"\n\t(\\w+)\t\t\t\t\t\t\t# function name\n\t(?:\\s+\n\t\t(of|in)\\s+\t\t\t\t\t# \"of\" or \"in\"\n\t\t(\\w+)\t\t\t\t\t\t# direct parameter\n\t)?\n\t(?=\\s+(above|against|apart\\s+from|around|aside\\s+from|at|below|beneath|beside|between|by|for|from|instead\\s+of|into|on|onto|out\\s+of|over|thru|under)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636700278792,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615341,
            b: 2251799813685248,
        },
    ]),(3, vec![
        Scope {
            a: 52636636700278792,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 49258876858335534,
            b: 2251799813685248,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 134 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(?x)\n\t\\s*(to|on)\\s+ \t\t\t\t\t# \"on\" or \"to\"\n\t(\\w+)\t\t\t\t\t\t\t# function name\n\t(?=\\s*(--.*?)?(?m:$))\t\t\t\t# nothing else"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636700278792,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615341,
            b: 2251799813685248,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 135 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 149 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 147 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 148 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 146 })),
]
} }