
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
      regex: Regex::new("([$@%*]#?)[`&\']"),
      scope: vec![
        Scope {
            a: 49259061525086947,
            b: 17169973579350016,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514493,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4324 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([$@%]#?)(?:[-+]|[1-9]+)"),
      scope: vec![
        Scope {
            a: 49259061525087476,
            b: 17169973579350016,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514493,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4324 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([$@%*]#?)(?x:\n  [_0abF]|ACCUMULATOR|ARG[CV]?|BASETIME|CHILD_ERROR|COMPILING|DEBUGGING|\n  EFFECTIVE_GROUP_ID|EFFECTIVE_USER_ID|EGID|ENV|ERRNO|EUID|EVAL_ERROR|\n  EXCEPTIONS_BEING_CAUGHT|EXECUTABLE_NAME|EXTENDED_OS_ERROR|FORMAT_FORMFEED|\n  FORMAT_LINE_BREAK_CHARACTERS|FORMAT_LINES_LEFT|FORMAT_LINES_PER_PAGE|\n  FORMAT_NAME|FORMAT_PAGE_NUMBER|FORMAT_TOP_NAME|GID|INPLACE_EDIT|\n  INPUT_LINE_NUMBER|INPUT_RECORD_SEPARATOR|INC|ISA|LAST_MATCH_END|\n  LAST_MATCH_START|LAST_PAREN_MATCH|LAST_REGEXP_CODE_RESULT|\n  LAST_SUBMATCH_RESULT|LIST_SEPARATOR|MATCH|NR|OFS|OLD_PERL_VERSION|ORS|\n  OS_ERROR|OSNAME|OUTPUT_AUTOFLUSH|OUTPUT_FIELD_SEPARATOR|\n  OUTPUT_RECORD_SEPARATOR|PERL_VERSION|PERLDB|PID|POSTMATCH|PREMATCH|\n  PROCESS_ID|PROGRAM_NAME|REAL_GROUP_ID|REAL_USER_ID|RS|SIG|\n  SUBSCRIPT_SEPARATOR|SUBSEP|SYSTEM_FD_MAX|UID|WARNING\n)\\b(?!::)"),
      scope: vec![
        Scope {
            a: 49259061526200320,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514493,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4324 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  ([$@%*]\\#?)\n  (?:\n    \\^[A-Z]? |\n    [\".:,;\\]~?!/\\\\|()<>] |\n    [$@%](?![-+])                # no regexp match\n  )(?![$@%\\w])\n)"),
      scope: vec![
        Scope {
            a: 49259061526200320,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514493,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4324 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([$@]\\#?)="),
      scope: vec![
        Scope {
            a: 49259061526200320,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514493,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4324 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([$@%*]#?)\\b[_\\p{L}]\\w*\\b"),
      scope: vec![
        Scope {
            a: 49259087310291005,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514493,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4324 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$)[#*\\[](?=[^\\w{$@%*])"),
      scope: vec![
        Scope {
            a: 49259061555429437,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514493,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4324 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([$@%*&]#?)(\\{)\\s*(\\b[_\\p{L}]\\w*\\b)\\s*(\\})"),
      scope: vec![
        Scope {
            a: 46444122780532736,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628181909565,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629322514614,
            b: 17169973579350016,
        },
    ]),(3, vec![
        Scope {
            a: 46444217269813248,
            b: 0,
        },
        Scope {
            a: 55451949100498944,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629322514603,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4324 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([$@%*&]#?)(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628181909565,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629322514614,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4324 }),
        ContextReference::Direct(ContextId { index: 4384 }),
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[$@%&]#?(?=[\\w$@%*]|::)"),
      scope: vec![
        Scope {
            a: 52636628181909565,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4294 }),
    ]),
      with_prototype: None
    }),
]
} }