
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
        a: 281496453775360,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281496453775360,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 2098 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2097 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2099 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(<\\?)(xml)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 1407374883553280,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122448,
            b: 1407374883553280,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2051 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(<)((?i:style))(?=[^[^\\t\\n\\f /<>]])"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46444230200328374,
            b: 1407374883553280,
        },
    ]),(1, vec![
        Scope {
            a: 47288629324153014,
            b: 1407374883553280,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632123125,
            b: 1407374883553280,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2119 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(<)((?i:script))(?=[^[^\\t\\n\\f /<>]])"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46444230169723062,
            b: 1407374883553280,
        },
    ]),(1, vec![
        Scope {
            a: 47288629324153014,
            b: 1407374883553280,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122658,
            b: 1407374883553280,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2111 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)((?i:body|head|html)(?=[^[^\\t\\n\\f /<>]]))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 1407374883553280,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632123099,
            b: 56857966770388992,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2052 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)((?ix:\n  address|applet|article|aside|blockquote|center|dd|dir|div|dl|dt|figcaption|figure|footer|frame|frameset|h1|h2|h3|h4|h5|h6|header|iframe|menu|nav|noframes|object|ol|p|pre|section|ul\n)(?=[^[^\\t\\n\\f /<>]]))"),
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
        ContextReference::Direct(ContextId { index: 2053 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)((?i:hr)(?=[^[^\\t\\n\\f /<>]]))"),
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
        ContextReference::Direct(ContextId { index: 2054 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)((?i:form|fieldset)(?=[^[^\\t\\n\\f /<>]]))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 1407374883553280,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122591,
            b: 211669203961249792,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2055 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)((?ix:\n  abbr|acronym|area|audio|b|base|basefont|bdi|bdo|big|br|canvas|caption|cite|code|del|details|dfn|dialog|em|font|head|html|i|img|ins|isindex|kbd|li|link|map|mark|menu|menuitem|meta|noscript|param|picture|q|rp|rt|rtc|ruby|s|samp|script|small|source|span|strike|strong|style|sub|summary|sup|time|title|track|tt|u|var|video|wbr\n)(?=[^[^\\t\\n\\f /<>]]))"),
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
        ContextReference::Direct(ContextId { index: 2056 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)((?ix:\n  button|datalist|input|label|legend|meter|optgroup|option|output|progress|select|template|textarea\n)(?=[^[^\\t\\n\\f /<>]]))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 1407374883553280,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632123121,
            b: 211669203961249792,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2057 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)((?i:a)(?=[^[^\\t\\n\\f /<>]]))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 1407374883553280,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632123121,
            b: 212232153914671104,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2058 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)((?i:col|colgroup|table|tbody|td|tfoot|th|thead|tr)(?=[^[^\\t\\n\\f /<>]]))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 1407374883553280,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632123121,
            b: 212513628891381760,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2059 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("</?(?=[A-Za-z][^\\t\\n\\f /<>]*?-)"),
      scope: vec![
        Scope {
            a: 47288629324153014,
            b: 1407374883553280,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2129 }),
        ContextReference::Direct(ContextId { index: 2130 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("</?(?=[A-Za-z])"),
      scope: vec![
        Scope {
            a: 47288629324153014,
            b: 1407374883553280,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2147 }),
        ContextReference::Direct(ContextId { index: 2148 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2105 })),
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
]
} }