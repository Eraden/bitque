
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
      regex: Regex::new("\\?(<[=!]|>|=|:|!)"),
      scope: vec![
        Scope {
            a: 59955136454393900,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4761 }),
        ContextReference::Direct(ContextId { index: 4774 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\?[ims]*x[ixms]*(?:-[ims]+)?)(\\))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439111721004,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636636696215596,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4753 }),
        ContextReference::Direct(ContextId { index: 4774 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\?[ims]*x[ixms]*(?:-[ims]+)?:"),
      scope: vec![
        Scope {
            a: 48414439111721004,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4762 }),
        ContextReference::Direct(ContextId { index: 4774 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\?(?:[ixms]*-)?[ixms]+:"),
      scope: vec![
        Scope {
            a: 48414439111721004,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4761 }),
        ContextReference::Direct(ContextId { index: 4774 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\?(?:(<)([^>]+)(>)|(\')([^\']+)(\'))"),
      scope: vec![
        Scope {
            a: 52636787100745772,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629398995126,
            b: 12384898975268864,
        },
    ]),(2, vec![
        Scope {
            a: 59392130707292204,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629398995115,
            b: 12384898975268864,
        },
    ]),(4, vec![
        Scope {
            a: 47288629398995126,
            b: 12384898975268864,
        },
    ]),(5, vec![
        Scope {
            a: 59392130707292204,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 47288629398995115,
            b: 12384898975268864,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4761 }),
        ContextReference::Direct(ContextId { index: 4774 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\?\\((?:(<)([^>]+)(>)|(\')([^\']+)(\')|(\\d+)|R(\\d*)|R&(\\w+))\\)"),
      scope: vec![
        Scope {
            a: 52636787100418398,
            b: 12384898975268864,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629398995126,
            b: 12384898975268864,
        },
    ]),(2, vec![
        Scope {
            a: 49259087379890220,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629398995115,
            b: 12384898975268864,
        },
    ]),(4, vec![
        Scope {
            a: 47288629398995126,
            b: 12384898975268864,
        },
    ]),(5, vec![
        Scope {
            a: 49259087379890220,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 47288629398995115,
            b: 12384898975268864,
        },
    ]),(7, vec![
        Scope {
            a: 49259087379890220,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 49259087379890220,
            b: 0,
        },
    ]),(9, vec![
        Scope {
            a: 49259087379890220,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4761 }),
        ContextReference::Direct(ContextId { index: 4774 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\?\\(DEFINE\\)"),
      scope: vec![
        Scope {
            a: 52636787035472066,
            b: 12384898975268864,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4761 }),
        ContextReference::Direct(ContextId { index: 4774 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\?(?=\\(\\?)"),
      scope: vec![
        Scope {
            a: 52636787035471916,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4761 }),
        ContextReference::Direct(ContextId { index: 4752 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4766 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 4761 }),
        ContextReference::Direct(ContextId { index: 4774 }),
    ]),
      with_prototype: None
    }),
]
} }