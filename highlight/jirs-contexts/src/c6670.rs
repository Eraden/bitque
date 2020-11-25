
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 844828657057792,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844828657057792,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n  ^\n  \\s*\n  (abstract)?\n  \\s*\n  (class|struct|union)\n  \\s+\n  (\n    (\n      [.A-Z_:\\x{80}-\\x{10FFFF}][.\\w:\\x{80}-\\x{10FFFF}]*\n      (\\(([,\\s.a-zA-Z0-9_:\\x{80}-\\x{10FFFF}]+)\\))?\n      (\n        \\s*(<)\\s*\n        [.:A-Z\\x{80}-\\x{10FFFF}][.:\\w\\x{80}-\\x{10FFFF}]*\n        (\\(([.a-zA-Z0-9_:]+\\s,)\\))?\n      )?\n    )|(\n      (<<)\n      \\s*\n      [.A-Z0-9_:\\x{80}-\\x{10FFFF}]+\n    )\n  )"),
      scope: vec![
        Scope {
            a: 46444243041779712,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636701982814,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636636701982814,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632450251,
            b: 26458647810801664,
        },
    ]),(5, vec![
        Scope {
            a: 47288620727271424,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 61925366769453135,
            b: 26458647810801664,
        },
    ]),(7, vec![
        Scope {
            a: 59392186470432862,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 47288620727271424,
            b: 0,
        },
    ]),(9, vec![
        Scope {
            a: 47288620727271424,
            b: 0,
        },
    ]),(10, vec![
        Scope {
            a: 61925366769453135,
            b: 26458647810801664,
        },
    ]),(11, vec![
        Scope {
            a: 47288629322514526,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(module)\\s+(([A-Z\\x{80}-\\x{10FFFF}][\\w\\x{80}-\\x{10FFFF}]*(::))?([A-Z\\x{80}-\\x{10FFFF}][\\w\\x{80}-\\x{10FFFF}]*(::))?([A-Z\\x{80}-\\x{10FFFF}][\\w\\x{80}-\\x{10FFFF}]*(::))*[A-Z\\x{80}-\\x{10FFFF}][\\w\\x{80}-\\x{10FFFF}]*)"),
      scope: vec![
        Scope {
            a: 46446648223465472,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636738682974,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632450811,
            b: 26458647810801664,
        },
    ]),(3, vec![
        Scope {
            a: 59392186470433531,
            b: 393783896145133568,
        },
    ]),(4, vec![
        Scope {
            a: 47288620785074270,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 59392186470433531,
            b: 598979154167201792,
        },
    ]),(6, vec![
        Scope {
            a: 47288620785074270,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 59392186470433531,
            b: 599260629143912448,
        },
    ]),(8, vec![
        Scope {
            a: 47288620785074270,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(lib)\\s+(([A-Z]\\w*(::))?([A-Z]\\w*(::))?([A-Z]\\w*(::))*[A-Z]\\w*)"),
      scope: vec![
        Scope {
            a: 46452519443759104,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636828270686,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632452178,
            b: 26458647810801664,
        },
    ]),(3, vec![
        Scope {
            a: 59392186470434898,
            b: 393783896145133568,
        },
    ]),(4, vec![
        Scope {
            a: 47288620785074270,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 59392186470434898,
            b: 598979154167201792,
        },
    ]),(6, vec![
        Scope {
            a: 47288620785074270,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 59392186470434898,
            b: 599260629143912448,
        },
    ]),(8, vec![
        Scope {
            a: 47288620785074270,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\belse(\\s)+if\\b"),
      scope: vec![
        Scope {
            a: 50104723409076224,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(BEGIN|alias|as|begin|case|select|abstract|class|END|ensure|for|fun|if|ifdef|in|lib|module|of|out|private|protected|rescue|struct|with|union|enum|macro|then|unless|until|while)\\b(?![?!])|(?<!\\.)\\btype\\b\\s*(?=[A-Z])"),
      scope: vec![
        Scope {
            a: 52636636828336222,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(when|else|elsif)\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636828401758,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(end)\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636828401835,
            b: 26458647810801664,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\bdo\\b\\s*"),
      scope: vec![
        Scope {
            a: 52636636775317598,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=\\{)(\\s+)"),
      scope: vec![
        Scope {
            a: 46445376913147178,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(and|not|or)\\b"),
      scope: vec![
        Scope {
            a: 52636628114800734,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b(alias|alias_method|break|next|pointerof|typeof|sizeof|instance_sizeof|return|super|yield|uninitialized|forall)\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636828467294,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(nil|true|false)\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 59955110643367936,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(__(DIR|FILE|LINE|END_LINE)__|self)\\b(?![?!])|self[?]"),
      scope: vec![
        Scope {
            a: 49259061528363008,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(initialize|new|loop|include|extend|raise|getter|setter|property|class_getter|class_setter|class_property|describe|context|it|with|delegate|def_hash|def_equals|def_equals_and_hash|forward_missing_to|record|assert_responds_to|spawn|annotation|verbatim)\\b[!?]?"),
      scope: vec![
        Scope {
            a: 52636636778397790,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(require)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636778397790,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6606 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@)[a-zA-Z_\\x{80}-\\x{10FFFF}][\\w\\x{80}-\\x{10FFFF}]*[?!=]?"),
      scope: vec![
        Scope {
            a: 49259087310291329,
            b: 26458647810801664,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514526,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@@)[a-zA-Z_\\x{80}-\\x{10FFFF}][\\w\\x{80}-\\x{10FFFF}]*[?!=]?"),
      scope: vec![
        Scope {
            a: 49259087310291147,
            b: 26458647810801664,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514526,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$)[a-zA-Z_]\\w*"),
      scope: vec![
        Scope {
            a: 49259087310291224,
            b: 26458647810801664,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514526,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$)(!|@|&|`|\'|\\+|\\d+|~|=|/|\\\\|,|;|\\.|<|>|_|\\*|\\$|\\?|:|\"|-[0adFiIlpv])"),
      scope: vec![
        Scope {
            a: 49259087310291224,
            b: 385058171867103232,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514526,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(ENV)\\["),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087305965662,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6607 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[A-Z\\x{80}-\\x{10FFFF}][\\w\\x{80}-\\x{10FFFF}]*"),
      scope: vec![
        Scope {
            a: 61925366760865792,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[A-Z\\x{80}-\\x{10FFFF}][\\w\\x{80}-\\x{10FFFF}]*\\b"),
      scope: vec![
        Scope {
            a: 49259087305965662,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?=def\\b)                                                      # an optimization to help Oniguruma fail fast\n (?<=^|\\s)(def)\\s+                                              # the def keyword\n ( (?>[a-zA-Z_\\x{80}-\\x{10FFFF}][\\x{80}-\\x{10FFFF}\\w]*(?>\\.|::))?                                   # a method name prefix\n   (?>[a-zA-Z_\\x{80}-\\x{10FFFF}][\\x{80}-\\x{10FFFF}\\w]*(?>[?!]|=(?!>))?                              # the method name\n   |===?|>[>=]?|<=>|<[<=]?|[%&`/\\|]|\\*\\*?|=?~|[-+]@?|\\[\\](?:=|\\?)?) )  # …or an operator method\n \\s*(\\()                                                        # the openning parenthesis for arguments"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636708601950,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615134,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629327560798,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6618 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?=def\\b)                                                      # an optimization to help Oniguruma fail fast\n (?<=^|\\s)(def)\\s+                                              # the def keyword\n ( (?>[a-zA-Z_\\x{80}-\\x{10FFFF}][\\w\\x{80}-\\x{10FFFF}]*(?>\\.|::))?                                   # a method name prefix\n   (?>[a-zA-Z_\\x{80}-\\x{10FFFF}][\\w\\x{80}-\\x{10FFFF}]*(?>[?!]|=(?!>))?                              # the method name\n   |===?|>[>=]?|<=>|<[<=]?|[%&`/\\|]|\\*\\*?|=?~|[-+]@?|\\[\\](?:=|\\?)?) )  # …or an operator method\n [ \\t]                                                          # the space separating the arguments\n (?=[ \\t]*[^\\s#;])                                              # make sure arguments and not a comment follow"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636708601950,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615134,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6629 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?=def\\b)                                                           # an optimization to help Oniguruma fail fast\n (?<=^|\\s)(def)\\b                                                    # the def keyword\n ( \\s+                                                               # an optional group of whitespace followed by…\n   ( (?>[a-zA-Z_\\x{80}-\\x{10FFFF}][\\w\\x{80}-\\x{10FFFF}]*(?>\\.|::))?                                      # a method name prefix\n     (?>[a-zA-Z_\\x{80}-\\x{10FFFF}][\\w\\x{80}-\\x{10FFFF}]*(?>[?!]|=(?!>))?                                 # the method name\n     |===?|>[>=]?|<=>|<[<=]?|[%&`/\\|]|\\*\\*?|=?~|[-+]@?|\\[\\](?:=|\\?)?) ) )?  # …or an operator method"),
      scope: vec![
        Scope {
            a: 46444131377350742,
            b: 26458647810801664,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636708601950,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130630615134,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(0[xX]\\h(?>_?\\h)*|\\d(?>_?\\d)*(\\.(?![^[:space:]\\p{Nd}])(?>_?\\d)*)?([eE][-+]?\\d(?>_?\\d)*)?|0[bB][01]+|0o[0-7]+)(_?(u8|u16|u32|u64|i8|i16|i32|i64|f32|f64))?\\b"),
      scope: vec![
        Scope {
            a: 59955089168531456,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(":\'"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629325004894,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6640 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(":\""),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629325004894,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6645 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("/="),
      scope: vec![
        Scope {
            a: 52636628111130982,
            b: 26458647810801664,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\'"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6646 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\""),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6647 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("`"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6648 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%x\\{"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6649 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%x\\["),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6608 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%x\\<"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6609 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%x\\("),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6610 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%x([^\\w])"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6611 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n   (?:\n     ^                      # beginning of line\n   | (?<=                   # or look-behind on:\n       [=>~(?:\\[,|&;]\n     | [\\s;]if\\s      # keywords\n     | [\\s;]elsif\\s\n     | [\\s;]while\\s\n     | [\\s;]unless\\s\n     | [\\s;]when\\s\n     | [\\s;]assert_match\\s\n     | [\\s;]or\\s      # boolean opperators\n     | [\\s;]and\\s\n     | [\\s;]not\\s\n     | [\\s.]index\\s     # methods\n     | [\\s.]scan\\s\n     | [\\s.]sub\\s\n     | [\\s.]sub!\\s\n     | [\\s.]gsub\\s\n     | [\\s.]gsub!\\s\n     | [\\s.]match\\s\n     )\n   | (?<=                  # or a look-behind with line anchor:\n        ^when\\s            # duplication necessary due to limits of regex\n      | ^if\\s\n      | ^elsif\\s\n      | ^while\\s\n      | ^unless\\s\n      )\n   )\n   \\s*((/))(?![*+{}?])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 55450759479820382,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956318,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6612 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%r\\{"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6613 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%r\\["),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6614 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%r\\("),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6615 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%r\\<"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6616 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%r([^\\w])"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6617 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%[QWSR]?\\("),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6619 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%[QWSR]?\\["),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6620 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%[QWSR]?\\<"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6621 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%[QWSR]?\\{"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6622 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%[QWSR]([^\\w])"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6623 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%[qws]\\("),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6624 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%[qws]\\<"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6625 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%[qws]\\["),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6626 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%[qws]\\{"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6627 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%[qws]([^\\w])"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6628 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!:)(:)(?>[a-zA-Z_\\x{80}-\\x{10FFFF}][\\w\\x{80}-\\x{10FFFF}]*(?>[?!]|=(?![>=]))?|===?|>[>=]?|<[<=]?|<=>|[%&`/\\|]|\\*\\*?|=?~|[-+]@?|\\[\\](?:=|\\?)?|@@?[a-zA-Z_\\x{80}-\\x{10FFFF}][\\w\\x{80}-\\x{10FFFF}]*|!=?(?![?!]))"),
      scope: vec![
        Scope {
            a: 59955136445349982,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325004894,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?>[a-zA-Z_\\x{80}-\\x{10FFFF}][\\w\\x{80}-\\x{10FFFF}]*(?>[?!])?)(:)(?!:)"),
      scope: vec![
        Scope {
            a: 59955136445349982,
            b: 600949075277250560,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325004894,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:^[ \\t]+)?(#).*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 51510711032873054,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323038814,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^__END__\\n"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 55451949185892446,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6630 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?><<-(\"?)((?:[_\\w]+_|)HTML)\\b\\1)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6632 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?><<-(\"?)((?:[_\\w]+_|)SQL)\\b\\1)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6633 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?><<-(\"?)((?:[_\\w]+_|)CSS)\\b\\1)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6634 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?><<-(\"?)((?:[_\\w]+_|)CPP)\\b\\1)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6635 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?><<-(\"?)((?:[_\\w]+_|)C)\\b\\1)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6636 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?><<-(\"?)((?:[_\\w]+_|)(?:JS|JAVASCRIPT))\\b\\1)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6637 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?><<-(\"?)((?:[_\\w]+_|)JQUERY)\\b\\1)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6638 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?><<-(\"?)((?:[_\\w]+_|)(?:SH|SHELL))\\b\\1)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6639 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?><<-(\"?)((?:[_\\w]+_|)RUBY)\\b\\1)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6641 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?>\\=\\s*<<(\\w+))"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6642 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?><<-(\\w+))"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 26458647810801664,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6643 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=\\{|do|\\{\\s|do\\s)(\\|)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620732579934,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6644 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("=>"),
      scope: vec![
        Scope {
            a: 47288620737429504,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<<=|%=|&=|\\*=|\\*\\*=|\\+=|\\-=|\\^=|\\|{1,2}=|<<"),
      scope: vec![
        Scope {
            a: 52636628111130982,
            b: 26458647810801664,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<=>|<(?!<|=)|>(?!<|=|>)|<=|>=|===|==|=~|!=|!~|(?<=[ \\t])\\?"),
      scope: vec![
        Scope {
            a: 52636628119257182,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=[ \\t])!+|\\bnot\\b|&&|\\band\\b|\\|\\||\\bor\\b|\\^"),
      scope: vec![
        Scope {
            a: 52636628114800734,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{\\%|\\%\\}|\\{\\{|\\}\\})"),
      scope: vec![
        Scope {
            a: 52636628128301150,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(%|&|\\*\\*|\\*|\\+|\\-|/)"),
      scope: vec![
        Scope {
            a: 52636628119191646,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("="),
      scope: vec![
        Scope {
            a: 52636628111130718,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\||~|>>"),
      scope: vec![
        Scope {
            a: 52636628113490014,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(":"),
      scope: vec![
        Scope {
            a: 47288620735856734,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\;"),
      scope: vec![
        Scope {
            a: 47288620734808158,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(","),
      scope: vec![
        Scope {
            a: 47288620775506014,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\.|::"),
      scope: vec![
        Scope {
            a: 47288620731990110,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{|\\}"),
      scope: vec![
        Scope {
            a: 47288522009083998,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\[|\\]"),
      scope: vec![
        Scope {
            a: 47288521948725342,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\(|\\)"),
      scope: vec![
        Scope {
            a: 47288521948463198,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }