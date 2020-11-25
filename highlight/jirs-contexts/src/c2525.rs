
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
  prototype: Some(
    ContextId {
        index: 2664,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("instanceof(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 52636628101562368,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2576 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("in(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))"),
      scope: vec![
        Scope {
            a: 52636628101562368,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2576 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("&&|\\|\\||\\?\\?"),
      scope: vec![
        Scope {
            a: 52636628114800683,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2576 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("=(?![=>])"),
      scope: vec![
        Scope {
            a: 52636628111130667,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2576 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n%=   | # assignment      right-to-left   both\n&=   | # assignment      right-to-left   both\n\\*=  | # assignment      right-to-left   both\n\\+=  | # assignment      right-to-left   both\n-=   | # assignment      right-to-left   both\n/=   | # assignment      right-to-left   both\n\\^=  | # assignment      right-to-left   both\n\\|=  | # assignment      right-to-left   both\n<<=  | # assignment      right-to-left   both\n>>=  | # assignment      right-to-left   both\n>>>=   # assignment      right-to-left   both"),
      scope: vec![
        Scope {
            a: 52636628111130982,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2576 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n<<   | # bitwise-shift   left-to-right   both\n>>>  | # bitwise-shift   left-to-right   both\n>>   | # bitwise-shift   left-to-right   both\n&    | # bitwise-and     left-to-right   both\n\\^   | # bitwise-xor     left-to-right   both\n\\|     # bitwise-or      left-to-right   both"),
      scope: vec![
        Scope {
            a: 52636628135903275,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2576 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n<=   | # relational      left-to-right   both\n>=   | # relational      left-to-right   both\n<    | # relational      left-to-right   both\n>      # relational      left-to-right   both"),
      scope: vec![
        Scope {
            a: 52636628152483883,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2576 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n===  | # equality        left-to-right   both\n!==  | # equality        left-to-right   both\n==   | # equality        left-to-right   both\n!=     # equality        left-to-right   both"),
      scope: vec![
        Scope {
            a: 52636628119257131,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2576 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n/    | # division        left-to-right   both\n%    | # modulus         left-to-right   both\n\\*   | # multiplication  left-to-right   both\n\\+   | # addition        left-to-right   both\n-      # subtraction     left-to-right   both"),
      scope: vec![
        Scope {
            a: 52636628119191595,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2576 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(","),
      scope: vec![
        Scope {
            a: 52636628134789163,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2576 }),
    ]),
      with_prototype: None
    }),
]
} }