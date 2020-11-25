
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
      regex: Regex::new("(?x)\n((@)(?:access|api))\n\\s+\n(private|protected|public)\n\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475834846,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325660360,
            b: 711005791171117056,
        },
    ]),(3, vec![
        Scope {
            a: 59955110803999198,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n((@)author)\n\\s+\n(\n  [^@\\s<>*/]\n  (?:[^@<>*/]|\\*[^/])*\n)\n(?:\n  \\s*\n  (<)\n  ([^>\\s]+)\n  (>)\n)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475834846,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325660360,
            b: 711005791171117056,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632450433,
            b: 711005791171117056,
        },
    ]),(4, vec![
        Scope {
            a: 47288629367997663,
            b: 51239294848729088,
        },
    ]),(5, vec![
        Scope {
            a: 59955136450789403,
            b: 114571164608626688,
        },
    ]),(6, vec![
        Scope {
            a: 47288629367997663,
            b: 48143070104911872,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n((@)borrows) \\s+\n((?:[^@\\s*/]|\\*[^/])+)    # <that namepath>\n\\s+ (as) \\s+              # as\n((?:[^@\\s*/]|\\*[^/])+)    # <this namepath>"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475834846,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325660360,
            b: 711005791171117056,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632450433,
            b: 711005791171117056,
        },
    ]),(4, vec![
        Scope {
            a: 52636628111198686,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 59392130632450433,
            b: 711005791171117056,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((@)example)\\s+"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475834846,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325660360,
            b: 711005791171117056,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9727 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x) ((@)kind) \\s+ (class|constant|event|external|file|function|member|mixin|module|namespace|typedef) \\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475834846,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325660360,
            b: 711005791171117056,
        },
    ]),(3, vec![
        Scope {
            a: 59955110669978078,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n((@)see)\n\\s+\n(?:\n  # URL\n  (\n    (?=https?://)\n    (?:[^\\s*]|\\*[^/])+\n  )\n  |\n  # JSDoc namepath\n  (\n    (?!\n      # Avoid matching bare URIs (also acceptable as links)\n      https?://\n      |\n      # Avoid matching {@inline tags}; we match those below\n      (?:\\[[^\\[\\]]*\\])? # Possible description [preceding]{@tag}\n      {@(?:link|linkcode|linkplain|tutorial)\\b\n    )\n    # Matched namepath\n    (?:[^@\\s*/]|\\*[^/])+\n  )\n)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475834846,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325660360,
            b: 711005791171117056,
        },
    ]),(3, vec![
        Scope {
            a: 49259087293776279,
            b: 711005791171117056,
        },
    ]),(4, vec![
        Scope {
            a: 59392130632450433,
            b: 711005791171117056,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n((@)template)\n\\s+\n# One or more valid identifiers\n(\n  [A-Za-z_$]         # First character: non-numeric word character\n  [\\w$.\\[\\]]*        # Rest of identifier\n  (?:                # Possible list of additional identifiers\n    \\s* , \\s*\n    [A-Za-z_$]\n    [\\w$.\\[\\]]*\n  )*\n)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475834846,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325660360,
            b: 711005791171117056,
        },
    ]),(3, vec![
        Scope {
            a: 49259087457550336,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(\n  (@)\n  (?:arg|argument|const|constant|member|namespace|param|var)\n)\n\\s+\n(\n  [A-Za-z_$]\n  [\\w$.\\[\\]]*\n)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475834846,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325660360,
            b: 711005791171117056,
        },
    ]),(3, vec![
        Scope {
            a: 49259087457550336,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((@)typedef)\\s+(?={)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475834846,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325660360,
            b: 711005791171117056,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9729 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((@)(?:arg|argument|const|constant|member|namespace|param|prop|property|var))\\s+(?={)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475834846,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325660360,
            b: 711005791171117056,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9730 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(\n  (@)\n  (?:define|enum|exception|export|extends|lends|implements|modifies\n  |namespace|private|protected|returns?|suppress|this|throws|type\n  |yields?)\n)\n\\s+(?={)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475834846,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325660360,
            b: 711005791171117056,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9731 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(\n  (@)\n  (?:alias|augments|callback|constructs|emits|event|fires|exports?\n  |extends|external|function|func|host|lends|listens|interface|memberof!?\n  |method|module|mixes|mixin|name|requires|see|this|typedef|uses)\n)\n\\s+\n(\n  (?:\n    [^{}@\\s*] | \\*[^/]\n  )+\n)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475834846,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325660360,
            b: 711005791171117056,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632450433,
            b: 711005791171117056,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((@)(?:default(?:value)?|license|version))\\s+(([\'\'\"]))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475834846,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325660360,
            b: 711005791171117056,
        },
    ]),(3, vec![
        Scope {
            a: 49259087457550336,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629323956406,
            b: 711005791171117056,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9732 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((@)(?:default(?:value)?|license|tutorial|variation|version))\\s+([^\\s*]+)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475834846,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325660360,
            b: 711005791171117056,
        },
    ]),(3, vec![
        Scope {
            a: 49259087457550336,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x) (@) (?:abstract|access|alias|api|arg|argument|async|attribute|augments|author|beta|borrows|bubbles |callback|chainable|class|classdesc|code|config|const|constant|constructor|constructs|copyright |default|defaultvalue|define|deprecated|desc|description|dict|emits|enum|event|example|exception |exports?|extends|extension(?:_?for)?|external|externs|file|fileoverview|final|fires|for|func |function|generator|global|hideconstructor|host|ignore|implements|implicitCast|inherit[Dd]oc |inner|instance|interface|internal|kind|lends|license|listens|main|member|memberof!?|method |mixes|mixins?|modifies|module|name|namespace|noalias|nocollapse|nocompile|nosideeffects |override|overview|package|param|polymer(?:Behavior)?|preserve|private|prop|property|protected |public|read[Oo]nly|record|require[ds]|returns?|see|since|static|struct|submodule|summary |suppress|template|this|throws|todo|tutorial|type|typedef|unrestricted|uses|var|variation |version|virtual|writeOnce|yields?) \\b"),
      scope: vec![
        Scope {
            a: 48414576475834846,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325660360,
            b: 711005791171117056,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9910 })),
]
} }