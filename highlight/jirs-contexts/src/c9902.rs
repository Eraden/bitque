
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9943 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(?:(?:(\\.)|(\\?\\.(?!\\s*[\\p{Nd}])))\\s*)?([_$\\p{L}][_$\\p{L}\\p{N}]*)(?=\\s*=\\s*(\n  ((async\\s+)?(\n    (function\\s*[(<*]) |\n    (function\\s+) |\n    ([_$\\p{L}][_$\\p{L}\\p{N}]*\\s*=>)\n  )) |\n  ((async\\s*)?(\n    ((<\\s*(?m:$))|([\\(]\\s*([\\{\\[]\\s*)?(?m:$))) |\n    # sure shot arrow functions even if => is on new line\n(\n  (<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<])*\\>)*>\\s*)?\n  [(]\\s*\n  (\n    ([)]\\s*:) |                                                                                       # ():\n    ((\\.\\.\\.\\s*)?[_$\\p{L}][_$\\p{L}\\p{N}]*\\s*:)                                                                  # [(]param: | [(]...param:\n  )\n) |\n(\n  [<]\\s*[_$\\p{L}][_$\\p{L}\\p{N}]*\\s+extends\\s*[^=>]                                                              # < typeparam extends\n) |\n# arrow function possible to detect only with => on same line\n(\n  (<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<])*\\>)*>\\s*)?                                                                                 # typeparameters\n  \\(\\s*(([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])|(\\.\\.\\.\\s*[_$\\p{L}]))([^()]|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\)))*)?\\)   # parameters\n  (\\s*:\\s*([^<>\\(\\)]|\\<[^<>]+\\>|\\([^\\(\\)]+\\))+)?                                                                        # return type\n  \\s*=>                                                                                               # arrow operator\n)\n  ))\n))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788234731520,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788293582999,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130630615191,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(\\.)|(\\?\\.(?!\\s*[\\p{Nd}])))\\s*([\\p{Lu}][_$\\p{Nd}\\p{Lu}]*)(?![_$\\p{L}\\p{N}])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788234731520,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788293582999,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49259087305965801,
            b: 42502721483309056,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(\\.)|(\\?\\.(?!\\s*[\\p{Nd}])))\\s*([_$\\p{L}][_$\\p{L}\\p{N}]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788234731520,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788293582999,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49259087307276439,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([\\p{Lu}][_$\\p{Nd}\\p{Lu}]*)(?![_$\\p{L}\\p{N}])"),
      scope: vec![
        Scope {
            a: 49259087305965719,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[_$\\p{L}][_$\\p{L}\\p{N}]*"),
      scope: vec![
        Scope {
            a: 49259087310291095,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }