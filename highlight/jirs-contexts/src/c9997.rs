
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
      regex: Regex::new("(?x)([_$\\p{L}][_$\\p{L}\\p{N}]*)(?=\\s*\n# function assignment |\n(=\\s*(\n  ((async\\s+)?(\n    (function\\s*[(<*]) |\n    (function\\s+) |\n    ([_$\\p{L}][_$\\p{L}\\p{N}]*\\s*=>)\n  )) |\n  ((async\\s*)?(\n    ((<\\s*(?m:$))|([\\(]\\s*([\\{\\[]\\s*)?(?m:$))) |\n    # sure shot arrow functions even if => is on new line\n(\n  (<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<])*\\>)*>\\s*)?\n  [(]\\s*\n  (\n    ([)]\\s*:) |                                                                                       # ():\n    ((\\.\\.\\.\\s*)?[_$\\p{L}][_$\\p{L}\\p{N}]*\\s*:)                                                                  # [(]param: | [(]...param:\n  )\n) |\n(\n  [<]\\s*[_$\\p{L}][_$\\p{L}\\p{N}]*\\s+extends\\s*[^=>]                                                              # < typeparam extends\n) |\n# arrow function possible to detect only with => on same line\n(\n  (<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<])*\\>)*>\\s*)?                                                                                 # typeparameters\n  \\(\\s*(([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])|(\\.\\.\\.\\s*[_$\\p{L}]))([^()]|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\)))*)?\\)   # parameters\n  (\\s*:\\s*([^<>\\(\\)]|\\<[^<>]+\\>|\\([^\\(\\)]+\\))+)?                                                                        # return type\n  \\s*=>                                                                                               # arrow operator\n)\n  ))\n)) |\n# typeannotation is fn type: < | () | (... | (param: | (param, | (param? | (param= | (param) =>\n(:\\s*(\n  (<) |\n  ([(]\\s*(\n    ([)]) |\n    (\\.\\.\\.) |\n    ([_$\\p{L}\\p{N}]+\\s*(\n      ([:,?=])|\n      ([)]\\s*=>)\n    ))\n  ))\n)) |\n(:\\s*((<\\s*(?m:$))|([\\(]\\s*([\\{\\[]\\s*)?(?m:$)))) |\n(:\\s*(=>|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(<[^<>]*>)|[^<>(),=])+=\\s*(\n  ((async\\s+)?(\n    (function\\s*[(<*]) |\n    (function\\s+) |\n    ([_$\\p{L}][_$\\p{L}\\p{N}]*\\s*=>)\n  )) |\n  ((async\\s*)?(\n    ((<\\s*(?m:$))|([\\(]\\s*([\\{\\[]\\s*)?(?m:$))) |\n    # sure shot arrow functions even if => is on new line\n(\n  (<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<])*\\>)*>\\s*)?\n  [(]\\s*\n  (\n    ([)]\\s*:) |                                                                                       # ():\n    ((\\.\\.\\.\\s*)?[_$\\p{L}][_$\\p{L}\\p{N}]*\\s*:)                                                                  # [(]param: | [(]...param:\n  )\n) |\n(\n  [<]\\s*[_$\\p{L}][_$\\p{L}\\p{N}]*\\s+extends\\s*[^=>]                                                              # < typeparam extends\n) |\n# arrow function possible to detect only with => on same line\n(\n  (<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<])*\\>)*>\\s*)?                                                                                 # typeparameters\n  \\(\\s*(([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])|(\\.\\.\\.\\s*[_$\\p{L}]))([^()]|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\)))*)?\\)   # parameters\n  (\\s*:\\s*([^<>\\(\\)]|\\<[^<>]+\\>|\\([^\\(\\)]+\\))+)?                                                                        # return type\n  \\s*=>                                                                                               # arrow operator\n)\n  ))\n)))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444204392382615,
            b: 0,
        },
        Scope {
            a: 49259087305965719,
            b: 0,
        },
        Scope {
            a: 59392130630615191,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9845 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([_$\\p{L}][_$\\p{L}\\p{N}]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444204392382615,
            b: 0,
        },
        Scope {
            a: 49259087305965719,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9846 }),
    ]),
      with_prototype: None
    }),
]
} }