
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9578 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9639 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\[)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9478 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=[\\\'\\\"\\`])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9479 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(?=(\\b(?<!\\$)0(?:x|X)[0-9a-fA-F][0-9a-fA-F_]*(n)?\\b(?!\\$))|(\\b(?<!\\$)0(?:b|B)[01][01_]*(n)?\\b(?!\\$))|(\\b(?<!\\$)0(?:o|O)?[0-7][0-7_]*(n)?\\b(?!\\$))|((?<!\\$)(?:\n  (?:\\b[0-9][0-9_]*(\\.)[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*(n)?\\b)| # 1.1E+3\n  (?:\\b[0-9][0-9_]*(\\.)[eE][+-]?[0-9][0-9_]*(n)?\\b)|             # 1.E+3\n  (?:\\B(\\.)[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*(n)?\\b)|             # .1E+3\n  (?:\\b[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*(n)?\\b)|                 # 1E+3\n  (?:\\b[0-9][0-9_]*(\\.)[0-9][0-9_]*(n)?\\b)|                      # 1.1\n  (?:\\b[0-9][0-9_]*(\\.)(n)?\\B)|                                  # 1.\n  (?:\\B(\\.)[0-9][0-9_]*(n)?\\b)|                                  # .1\n  (?:\\b[0-9][0-9_]*(n)?\\b(?!\\.))                                 # 1\n)(?!\\$)))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9483 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=[\\]\\\'\\\"\\`])(?=\\s*[\\(\\<])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9484 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?![_$\\p{L}])([\\p{Nd}]+)\\s*(?=(\\/\\*([^\\*]|(\\*[^\\/]))*\\*\\/\\s*)*:)"),
      scope: vec![
        Scope {
            a: 46446935994990742,
            b: 0,
        },
    ],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46454254623719574,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 59955089176658070,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(?:([_$\\p{L}][_$\\p{L}\\p{N}]*)\\s*(?=(\\/\\*([^\\*]|(\\*[^\\/]))*\\*\\/\\s*)*:(\\s*\\/\\*([^\\*]|(\\*[^\\/]))*\\*\\/)*\\s*(\n  ((async\\s+)?(\n    (function\\s*[(<*]) |\n    (function\\s+) |\n    ([_$\\p{L}][_$\\p{L}\\p{N}]*\\s*=>)\n  )) |\n  ((async\\s*)?(\n    ((<\\s*(?m:$))|((<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<])*\\>)*\\>)*>\\s*)?[\\(]\\s*((([\\{\\[]\\s*)?(?m:$))|((\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})\\s*((:\\s*\\{?(?m:$))|((\\s*([^<>\\(\\)\\{\\}]|\\<([^<>]|\\<[^<>]+\\>)+\\>|\\([^\\(\\)]+\\)|\\{[^\\{\\}]+\\})+\\s*)?=\\s*)))|((\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])\\s*((:\\s*\\[?(?m:$))|((\\s*([^<>\\(\\)\\{\\}]|\\<([^<>]|\\<[^<>]+\\>)+\\>|\\([^\\(\\)]+\\)|\\{[^\\{\\}]+\\})+\\s*)?=\\s*)))))) |\n    # sure shot arrow functions even if => is on new line\n(\n  (<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<])*\\>)*\\>)*>\\s*)?\n  [(]\\s*(\\/\\*([^\\*]|(\\*[^\\/]))*\\*\\/\\s*)*\n  (\n    ([)]\\s*:) |                                                                                       # ():\n    ((\\.\\.\\.\\s*)?[_$\\p{L}][_$\\p{L}\\p{N}]*\\s*:)                                                                  # [(]param: | [(]...param:\n  )\n) |\n\n# arrow function possible to detect only with => on same line\n(\n  (<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<])*\\>)*\\>)*>\\s*)?                                                                                 # typeparameters\n  \\(\\s*(\\/\\*([^\\*]|(\\*[^\\/]))*\\*\\/\\s*)*(([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])|(\\.\\.\\.\\s*[_$\\p{L}]))([^()\\\'\\\"\\`]|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\\'([^\\\'\\\\]|\\\\.)*\\\')|(\\\"([^\\\"\\\\]|\\\\.)*\\\")|(\\`([^\\`\\\\]|\\\\.)*\\`))*)?\\)   # parameters\n  (\\s*:\\s*([^<>\\(\\)\\{\\}]|\\<([^<>]|\\<[^<>]+\\>)+\\>|\\([^\\(\\)]+\\)|\\{[^\\{\\}]+\\})+)?                                                                        # return type\n  \\s*=>                                                                                               # arrow operator\n)\n  ))\n)))"),
      scope: vec![
        Scope {
            a: 46446935994990742,
            b: 0,
        },
    ],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46454254623719574,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 59392130630615190,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:[_$\\p{L}][_$\\p{L}\\p{N}]*)\\s*(?=(\\/\\*([^\\*]|(\\*[^\\/]))*\\*\\/\\s*)*:)"),
      scope: vec![
        Scope {
            a: 46446935994990742,
            b: 0,
        },
    ],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46454254623719574,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\.\\.\\."),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 52636628152877206,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9485 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([_$\\p{L}][_$\\p{L}\\p{N}]*)\\s*(?=,|\\}|(?m:$)|\\/\\/|\\/\\*)"),
      scope: vec![
        Scope {
            a: 46446935994990742,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087310291094,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(as)\\s+(const)(?=\\s*([,}]|(?m:$)))"),
      scope: vec![
        Scope {
            a: 46446935994990742,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636736848022,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414439033405440,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(as)\\s+"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636736848022,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9486 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=[_$\\p{L}][_$\\p{L}\\p{N}]*\\s*=)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9487 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(":"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46454254623719574,
            b: 0,
        },
        Scope {
            a: 47288620737429654,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9488 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9653 })),
]
} }