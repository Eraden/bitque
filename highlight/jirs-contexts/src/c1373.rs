
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
        a: 844506534510592,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844506534510592,
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
      regex: Regex::new("^((\\*{15})|(={67})|(-{3}))(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 46444195792224256,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323628563,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\d+(,\\d+)*(a|d|c)\\d+(,\\d+)*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 46443452779528765,
            b: 0,
        },
        Scope {
            a: 46444543720882195,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(@@)\\s*(.+?)\\s*(@@)\\s*(.*?)\\s*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 46443452779528766,
            b: 0,
        },
        Scope {
            a: 46444543720947731,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629328936979,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46445273865191443,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629328936979,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130630090771,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(((\\-{3}) .+ (\\-{4}))|((\\*{3}) .+ (\\*{4})))(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 46443452779528767,
            b: 0,
        },
        Scope {
            a: 46444543721013267,
            b: 0,
        },
    ],
      captures: Some(vec![(3, vec![
        Scope {
            a: 47288629328936979,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629328936979,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 47288629328936979,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 47288629328936979,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(^(((-{3}) .+)|((\\*{3}) .+))(?m:$)\\n?|^(={4}) .+(?= - ))"),
      scope: vec![
        Scope {
            a: 46443452799386177,
            b: 0,
        },
        Scope {
            a: 46445845096235027,
            b: 0,
        },
    ],
      captures: Some(vec![(4, vec![
        Scope {
            a: 47288629348859923,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 47288629348859923,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 47288629348859923,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(^(\\+{3}) .+(?m:$)\\n?| (-) .* (={4})(?m:$)\\n?)"),
      scope: vec![
        Scope {
            a: 46443452799386178,
            b: 0,
        },
        Scope {
            a: 46445845096300563,
            b: 0,
        },
    ],
      captures: Some(vec![(2, vec![
        Scope {
            a: 47288629348925459,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629348925459,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629348925459,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(((>)( .*)?)|((\\+).*))(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 114281327331835904,
            b: 0,
        },
    ],
      captures: Some(vec![(3, vec![
        Scope {
            a: 47288629348990995,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 47288629348990995,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(!).*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 114281331626803200,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629349056531,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(((<)( .*)?)|((-).*))(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 114281335921770496,
            b: 0,
        },
    ],
      captures: Some(vec![(3, vec![
        Scope {
            a: 47288629349122067,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 47288629349122067,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^Index(:) (.+)(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 46443452799778816,
            b: 0,
        },
        Scope {
            a: 46445870829469696,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620737429523,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46445273865977875,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }