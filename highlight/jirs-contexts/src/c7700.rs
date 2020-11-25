
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
        a: 281496459149312,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281496459149312,
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
      regex: Regex::new("(<)([a-zA-Z0-9:]++)(?=[^>]*></\\2>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324152837,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122373,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7659 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(<\\?)(xml)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324152837,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122448,
            b: 1407374883553280,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7660 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<!--"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323038725,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7666 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<!"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629324152837,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7667 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7698 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:^\\s+)?(<)((?i:style))\\b(?![^>]*/>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324152837,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632123125,
            b: 1407374883553280,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7670 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:^\\s+)?(<)((?i:script))\\b(?![^>]*/>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324152837,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122658,
            b: 1407374883553280,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7672 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)((?i:body|head|html)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324152837,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632123099,
            b: 56857966770388992,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7662 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)((?i:address|blockquote|dd|div|dl|dt|fieldset|form|frame|frameset|h1|h2|h3|h4|h5|h6|iframe|noframes|object|ol|p|ul|applet|center|dir|hr|menu|pre)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 1407374883553280,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122591,
            b: 56857966770388992,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7663 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)((?i:a|abbr|acronym|area|b|base|basefont|bdo|big|br|button|caption|cite|code|col|colgroup|del|dfn|em|font|head|html|i|img|input|ins|isindex|kbd|label|legend|li|link|map|meta|noscript|optgroup|option|param|q|s|samp|script|select|small|span|strike|strong|style|sub|sup|table|tbody|td|textarea|tfoot|th|thead|title|tr|tt|u|var)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 1407374883553280,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632123121,
            b: 56857966770388992,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7664 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)([a-zA-Z0-9:]+)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 1407374883553280,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122593,
            b: 1407374883553280,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7665 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7699 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<>"),
      scope: vec![
        Scope {
            a: 50103314684641285,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<"),
      scope: vec![
        Scope {
            a: 50103314807390213,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7723 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7725 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7710 })),
]
} }