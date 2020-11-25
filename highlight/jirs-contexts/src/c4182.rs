
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
        a: 844682628169728,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844682628169728,
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
      regex: Regex::new("\\b(?i:(absolute|abstract|all|and|and_then|array|as|asm|attribute|begin|bindable|case|class|const|constructor|destructor|div|do|do|else|end|except|export|exports|external|far|file|finalization|finally|for|forward|goto|if|implementation|import|in|inherited|initialization|interface|interrupt|is|label|library|mod|module|name|near|nil|not|object|of|only|operator|or|or_else|otherwise|packed|pow|private|program|property|protected|public|published|qualified|record|repeat|resident|restricted|segment|set|shl|shr|then|to|try|type|unit|until|uses|value|var|view|virtual|while|with|xor))\\b"),
      scope: vec![
        Scope {
            a: 52636636692611072,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:(function|procedure))\\b\\s+(\\w+(\\.\\w+)?)(\\(.*?\\))?;\\s*(?=(?i:attribute|forward|external))"),
      scope: vec![
        Scope {
            a: 46444131420536892,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576516595772,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615865,
            b: 16888498602639360,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:(function|procedure))\\b\\s+(\\w+(\\.\\w+)?)"),
      scope: vec![
        Scope {
            a: 46444131370401792,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576474128444,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615100,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((0(x|X)[0-9a-fA-F]*)|(([0-9]+\\.?[0-9]*)|(\\.[0-9]+))((e|E)(\\+|-)?[0-9]+)?)(L|l|UL|ul|u|U|F|f|ll|LL|ull|ULL)?\\b"),
      scope: vec![
        Scope {
            a: 59955089166303232,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(^[ \\t]+)?(--)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288466114282589,
            b: 16888498602639360,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323038780,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4174 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(^[ \\t]+)?(//)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288466114282589,
            b: 16888498602639360,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323038780,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4175 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\(\\*"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323038780,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4176 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323038780,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4177 }),
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
            b: 16888498602639360,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4178 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\'"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 16888498602639360,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4179 }),
    ]),
      with_prototype: None
    }),
]
} }