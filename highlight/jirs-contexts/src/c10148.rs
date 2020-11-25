
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
        a: 281496461639680,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281496461639680,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 10196 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10149 })),
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
        ContextReference::Direct(ContextId { index: 10037 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(<!--)(-?>)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323038902,
            b: 1407374883553280,
        },
    ]),(2, vec![
        Scope {
            a: 50103314702794757,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10038 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<!"),
      scope: vec![
        Scope {
            a: 47288629324152837,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10045 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)([a-z_][a-z0-9:_]*-[a-z0-9:_-]+)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 1407374883553280,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122880,
            b: 1407374883553280,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10048 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(<)((?i:style))\\b"),
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
        ContextReference::Direct(ContextId { index: 10163 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(<)((?i:script))\\b"),
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
        ContextReference::Direct(ContextId { index: 10153 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)((?i:body|head|html)\\b)"),
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
        ContextReference::Direct(ContextId { index: 10049 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)((?ix)(?:\n  address|applet|article|aside|blockquote|center|dd|dir|div|dl|dt|figcaption|figure|footer|frame|frameset|h1|h2|h3|h4|h5|h6|header|iframe|menu|nav|noframes|object|ol|p|pre|section|ul\n)\\b)"),
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
        ContextReference::Direct(ContextId { index: 10050 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)((?i:hr)\\b)"),
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
        ContextReference::Direct(ContextId { index: 10051 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)((?i:form|fieldset)\\b)"),
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
        ContextReference::Direct(ContextId { index: 10052 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)((?ix)(?:\n  abbr|acronym|area|audio|b|base|basefont|bdi|bdo|big|br|canvas|caption|cite|code|del|details|dfn|dialog|em|font|head|html|i|img|ins|isindex|kbd|li|link|map|mark|menu|menuitem|meta|noscript|param|picture|q|rp|rt|rtc|ruby|s|samp|script|small|source|span|strike|strong|style|sub|summary|sup|time|title|track|tt|u|var|video|wbr\n)\\b)"),
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
        ContextReference::Direct(ContextId { index: 10039 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)((?ix)(?:\n  button|datalist|input|label|legend|meter|optgroup|option|output|progress|select|template|textarea\n)\\b)"),
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
        ContextReference::Direct(ContextId { index: 10040 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)((?i:a)\\b)"),
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
        ContextReference::Direct(ContextId { index: 10041 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)((?i:col|colgroup|table|tbody|td|tfoot|th|thead|tr)\\b)"),
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
        ContextReference::Direct(ContextId { index: 10042 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)([A-Za-z0-9:_]+-[A-Za-z0-9:_-]+)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 1407374883553280,
        },
    ]),(2, vec![
        Scope {
            a: 50103314822463493,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10043 }),
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
        ContextReference::Direct(ContextId { index: 10044 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10143 })),
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