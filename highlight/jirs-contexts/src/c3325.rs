
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
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:xml))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 22522009636306944,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3220 }),
        ContextReference::Direct(ContextId { index: 5819 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3221 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:sql))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 19425784892489728,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3232 }),
        ContextReference::Direct(ContextId { index: 5102 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3243 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:python|py))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 17455460055515136,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3254 }),
        ContextReference::Direct(ContextId { index: 4613 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3256 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:graphviz))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 266279339467735040,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3257 }),
        ContextReference::Direct(ContextId { index: 1984 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3258 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:javascript|js))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 266560814444445696,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3259 }),
        ContextReference::Direct(ContextId { index: 7848 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3260 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:json))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 10700060614459392,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3222 }),
        ContextReference::Direct(ContextId { index: 2200 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3223 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:java))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 11263010567880704,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3224 }),
        ContextReference::Direct(ContextId { index: 2353 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3225 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:csharp|c\\#|cs))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 265434914537603072,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3226 }),
        ContextReference::Direct(ContextId { index: 383 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3227 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:rust))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 20270209822621696,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3228 }),
        ContextReference::Direct(ContextId { index: 5068 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3229 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:shell-script|sh|bash|zsh))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 265716389514313728,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3230 }),
        ContextReference::Direct(ContextId { index: 5430 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3231 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:php|inc))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 16329560148672512,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3233 }),
        ContextReference::Direct(ContextId { index: 4135 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3234 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:html\\+php))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 265997864491024384,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3235 }),
        ContextReference::Direct(ContextId { index: 4163 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3236 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:rscript|r|splus))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 18018410008936448,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3237 }),
        ContextReference::Direct(ContextId { index: 4674 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3238 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:golang))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 8729735777484800,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3239 }),
        ContextReference::Direct(ContextId { index: 1879 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3240 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:ruby|rb|rbx))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 18581359962357760,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3241 }),
        ContextReference::Direct(ContextId { index: 4920 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3242 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:objc|obj-c|objectivec|objective-c))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 16048085171961856,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3244 }),
        ContextReference::Direct(ContextId { index: 3925 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3245 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:objc\\+\\+|obj-c\\+\\+|objectivec\\+\\+|objective-c\\+\\+))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 15766610195251200,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3246 }),
        ContextReference::Direct(ContextId { index: 3754 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3247 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:c))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 3663186196692992,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3248 }),
        ContextReference::Direct(ContextId { index: 777 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3249 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:c\\+\\+|cpp))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 3381711219982336,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3250 }),
        ContextReference::Direct(ContextId { index: 610 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3251 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ((?i:regexp|regex))\n (?x:\n  (\n    \\s*        # any whitespace, or ..\n  |\n    \\s[^`]*    # any characters (except backticks), separated by whitespace ...\n  )\n  (?m:$)\\n?         # ... until EOL\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 12388910474723328,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3252 }),
        ContextReference::Direct(ContextId { index: 4769 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 3253 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n (?x:\n  ([ \\t]*)\n  (\n    (`){3,}    #   3 or more backticks\n    (?![^`]*`) #   not followed by any more backticks on the same line\n  |            # or\n    (~){3,}    #   3 or more tildas\n    (?![^~]*~) #   not followed by any more tildas on the same line\n  )\n  \\s*          # allow for whitespace between code block start and info string\n)\n ([\\w-]*)     # any number of word characters or dashes\n .*(?m:$)\\n?       # all characters until EOL"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46447417029165238,
            b: 285486476165120,
        },
    ]),(2, vec![
        Scope {
            a: 47288629330576302,
            b: 51228656214736896,
        },
    ]),(5, vec![
        Scope {
            a: 59955136470515761,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3255 }),
    ]),
      with_prototype: None
    }),
]
} }