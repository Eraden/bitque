
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
      regex: Regex::new("(?x)(?=[};,=]|(?m:$)|(^(?!\\s*((\\b(?<!\\$)0(x|X)[0-9a-fA-F][0-9a-fA-F_]*\\b(?!\\$))|(\\b(?<!\\$)0(b|B)[01][01_]*\\b(?!\\$))|(\\b(?<!\\$)0(o|O)?[0-7][0-7_]*\\b(?!\\$))|((?<!\\$)(?:\n  (?:\\b[0-9][0-9_]*(\\.)[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*\\b)| # 1.1E+3\n  (?:\\b[0-9][0-9_]*(\\.)[eE][+-]?[0-9][0-9_]*\\b)|             # 1.E+3\n  (?:\\B(\\.)[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*\\b)|             # .1E+3\n  (?:\\b[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*\\b)|                 # 1E+3\n  (?:\\b[0-9][0-9_]*(\\.)[0-9][0-9_]*\\b)|                      # 1.1\n  (?:\\b[0-9][0-9_]*(\\.)\\B)|                                  # 1.\n  (?:\\B(\\.)[0-9][0-9_]*\\b)|                                  # .1\n  (?:\\b[0-9][0-9_]*\\b(?!\\.))                                 # 1\n)(?!\\$))|([_$\\p{L}][_$\\p{L}\\p{N}]*)|(\\\'([^\\\'\\\\]|\\\\\\\'|\\\\)*\\\')|(\\\"([^\\\"\\\\]|\\\\\\\"|\\\\)*\\\")|(\\[([^\\[\\]]|\\[[^\\[\\]]*\\])+\\]))\\s*(\\?\\s*)?(=|:))))|(?<=\\})"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9979 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9968 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9858 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9936 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9873 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)([_$\\p{L}][_$\\p{L}\\p{N}]*)(\\?)?(?=(\\?\\s*)?\\s*\n# function assignment |\n(=\\s*(\n  ((async\\s+)?(\n    (function\\s*[(<*]) |\n    (function\\s+) |\n    ([_$\\p{L}][_$\\p{L}\\p{N}]*\\s*=>)\n  )) |\n  ((async\\s*)?(\n    ((<\\s*(?m:$))|([\\(]\\s*([\\{\\[]\\s*)?(?m:$))) |\n    # sure shot arrow functions even if => is on new line\n(\n  (<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<])*\\>)*>\\s*)?\n  [(]\\s*\n  (\n    ([)]\\s*:) |                                                                                       # ():\n    ((\\.\\.\\.\\s*)?[_$\\p{L}][_$\\p{L}\\p{N}]*\\s*:)                                                                  # [(]param: | [(]...param:\n  )\n) |\n(\n  [<]\\s*[_$\\p{L}][_$\\p{L}\\p{N}]*\\s+extends\\s*[^=>]                                                              # < typeparam extends\n) |\n# arrow function possible to detect only with => on same line\n(\n  (<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<])*\\>)*>\\s*)?                                                                                 # typeparameters\n  \\(\\s*(([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])|(\\.\\.\\.\\s*[_$\\p{L}]))([^()]|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\)))*)?\\)   # parameters\n  (\\s*:\\s*([^<>\\(\\)]|\\<[^<>]+\\>|\\([^\\(\\)]+\\))+)?                                                                        # return type\n  \\s*=>                                                                                               # arrow operator\n)\n  ))\n)) |\n# typeannotation is fn type: < | () | (... | (param: | (param, | (param? | (param= | (param) =>\n(:\\s*(\n  (<) |\n  ([(]\\s*(\n    ([)]) |\n    (\\.\\.\\.) |\n    ([_$\\p{L}\\p{N}]+\\s*(\n      ([:,?=])|\n      ([)]\\s*=>)\n    ))\n  ))\n)) |\n(:\\s*((<\\s*(?m:$))|([\\(]\\s*([\\{\\[]\\s*)?(?m:$)))) |\n(:\\s*(=>|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(<[^<>]*>)|[^<>(),=])+=\\s*(\n  ((async\\s+)?(\n    (function\\s*[(<*]) |\n    (function\\s+) |\n    ([_$\\p{L}][_$\\p{L}\\p{N}]*\\s*=>)\n  )) |\n  ((async\\s*)?(\n    ((<\\s*(?m:$))|([\\(]\\s*([\\{\\[]\\s*)?(?m:$))) |\n    # sure shot arrow functions even if => is on new line\n(\n  (<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<])*\\>)*>\\s*)?\n  [(]\\s*\n  (\n    ([)]\\s*:) |                                                                                       # ():\n    ((\\.\\.\\.\\s*)?[_$\\p{L}][_$\\p{L}\\p{N}]*\\s*:)                                                                  # [(]param: | [(]...param:\n  )\n) |\n(\n  [<]\\s*[_$\\p{L}][_$\\p{L}\\p{N}]*\\s+extends\\s*[^=>]                                                              # < typeparam extends\n) |\n# arrow function possible to detect only with => on same line\n(\n  (<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<])*\\>)*>\\s*)?                                                                                 # typeparameters\n  \\(\\s*(([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])|(\\.\\.\\.\\s*[_$\\p{L}]))([^()]|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\)))*)?\\)   # parameters\n  (\\s*:\\s*([^<>\\(\\)]|\\<[^<>]+\\>|\\([^\\(\\)]+\\))+)?                                                                        # return type\n  \\s*=>                                                                                               # arrow operator\n)\n  ))\n)))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444204396183703,
            b: 0,
        },
        Scope {
            a: 59392130630615191,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628167491735,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[_$\\p{L}][_$\\p{L}\\p{N}]*"),
      scope: vec![
        Scope {
            a: 46444204396183703,
            b: 0,
        },
        Scope {
            a: 49261685762490519,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\?"),
      scope: vec![
        Scope {
            a: 52636628167491735,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }