
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
      regex: Regex::new("\\bassert\\b"),
      scope: vec![
        Scope {
            a: 52636636725117024,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bas\\b"),
      scope: vec![
        Scope {
            a: 52637435558887424,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(try|catch|finally|throw|on|rethrow)\\b"),
      scope: vec![
        Scope {
            a: 52636636829384800,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s+\\?\\s+|\\s+:\\s+"),
      scope: vec![
        Scope {
            a: 52636636713255008,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(break|case|continue|default|do|else|for|if|in|return|switch|while)\\b"),
      scope: vec![
        Scope {
            a: 52636636694970368,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(async\\*|async|await\\*|await|yield)\\b"),
      scope: vec![
        Scope {
            a: 52636636773285984,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(new)\\b"),
      scope: vec![
        Scope {
            a: 52636636703686752,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(abstract|extends|external|factory|implements|with|interface|get|native|operator|set|typedef)\\b"),
      scope: vec![
        Scope {
            a: 52638212947968000,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(is\\!?)\\b"),
      scope: vec![
        Scope {
            a: 52636628105035776,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(<<|>>>?|~|\\^|\\||&)"),
      scope: vec![
        Scope {
            a: 52636628135903328,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((&|\\^|\\||<<|>>>?)=)"),
      scope: vec![
        Scope {
            a: 52636628111131191,
            b: 27021597764222976,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(===?|!==?|<=?|>=?)"),
      scope: vec![
        Scope {
            a: 52636628119257184,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(([+*/%-]|\\~)=)"),
      scope: vec![
        Scope {
            a: 52636628111130936,
            b: 27021597764222976,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(=)"),
      scope: vec![
        Scope {
            a: 52636628111130720,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\-\\-|\\+\\+)"),
      scope: vec![
        Scope {
            a: 52636628147372128,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\-|\\+|\\*|\\/|\\~\\/|%)"),
      scope: vec![
        Scope {
            a: 52636628119191648,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(!|&&|\\|\\|)"),
      scope: vec![
        Scope {
            a: 52636628114800736,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(";"),
      scope: vec![
        Scope {
            a: 47288689446879232,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(static|final|const)\\b"),
      scope: vec![
        Scope {
            a: 48414439029866496,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:void|bool|num|int|double|dynamic|var|String|List|Map)\\b"),
      scope: vec![
        Scope {
            a: 48414576511352928,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }