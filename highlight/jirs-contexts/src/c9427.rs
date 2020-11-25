
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
      regex: Regex::new("(?=\\s|\\*/|[^{}\\[\\]A-Za-z_$])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9621 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([A-Za-z_$][\\w$.\\[\\]]*)"),
      scope: vec![
        Scope {
            a: 49259087457550336,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(\\[)\\s*\n[\\w$]+\n(?:\n  (?:\\[\\])?                                        # Foo[ ].bar properties within an array\n  \\.                                                # Foo.Bar namespaced parameter\n  [\\w$]+\n)*\n(?:\n  \\s*\n  (=)                                                # [foo=bar] Default parameter value\n  \\s*\n  (\n    # The inner regexes are to stop the match early at */ and to not stop at escaped quotes\n    (?>\n      \"(?:(?:\\*(?!/))|(?:\\\\(?!\"))|[^*\\\\])*?\" |                      # [foo=\"bar\"] Double-quoted\n      \'(?:(?:\\*(?!/))|(?:\\\\(?!\'))|[^*\\\\])*?\' |                      # [foo=\'bar\'] Single-quoted\n      \\[ (?:(?:\\*(?!/))|[^*])*? \\] |                                # [foo=[1,2]] Array literal\n      (?:(?:\\*(?!/))|\\s(?!\\s*\\])|\\[.*?(?:\\]|(?=\\*/))|[^*\\s\\[\\]])*   # Everything else\n    )*\n  )\n)?\n\\s*(?:(\\])((?:[^*\\s]|\\*[^\\s/])+)?|(?=\\*/))"),
      scope: vec![
        Scope {
            a: 49259087457550336,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629476655286,
            b: 244607316909752320,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111133150,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 845262458585088,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629476655275,
            b: 244607316909752320,
        },
    ]),(5, vec![
        Scope {
            a: 50103314684250590,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }