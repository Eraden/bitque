
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
        a: 845047700389888,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 845047700389888,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 9204 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(@(?:import|charset|css|font-face|(?:-webkit-)?keyframes)(?:\\s+([\\w-]+))?)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636720398561,
            b: 40813871623045120,
        },
    ]),(2, vec![
        Scope {
            a: 49259087327330449,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9183 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(@media)\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636720398829,
            b: 40813871623045120,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9184 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(?<=^|;|})\n\\s*\n(?=\n    [\\[\\]\'\".\\w$-]+\n    \\s*\n    ([?:]?=)\n    (?![^\\[]*\\])\n)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9185 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9212 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9205 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9226 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)            # multi-line regex definition mode\n\n^(\\s*)          # starts at the beginning of line\n([\\w$-]+)       # identifier (name)\n(\\()            # start of argument list\n(?=\n    .*?\n    \\)\\s*\\{     # we see a curly brace afterwards\n)               # which means this is a function definition"),
      scope: vec![],
      captures: Some(vec![(2, vec![
        Scope {
            a: 59392130630615185,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629327561613,
            b: 40813871623045120,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9186 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)                # multi-line regex definition mode\n(\n\n    (^|;)           # starts at the beginning of line or at a ;\n    \\s*\n    (\\+?\\s*         # for block mixins\n     [\\w$-]+)       # identifier (name)\n    (\\()            # start of argument list\n    (?=\n        .*?\n        \\)\\s*;?\\s*  # if there are only spaces and semicolons\n        (?m:$)|;         # then this a\n    )\n)"),
      scope: vec![],
      captures: Some(vec![(3, vec![
        Scope {
            a: 59392186477185360,
            b: 40813871623045120,
        },
    ]),(4, vec![
        Scope {
            a: 47288629327561613,
            b: 40813871623045120,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9187 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x) # multi-line regex definition mode\n(^|(?<=\\*/|\\}))\\s*\n(?=\n    font(?!\n        \\s*:\\s\n        |\n        -\n        |\n        .*?\n        (?:\n            \\/|normal|bold|light(er?)|serif|sans|monospace|\n            \\b\\d+(?:\\b|px|r?em|%)|\n            var\\s*\\(|\n            [\'\"][^\\]]*(?m:$)\n        )\n    ) | # we need to distinguish between tag and property `cursor`\n    cursor(?!\n        \\s*[:;]\\s\n        |\n        -\n        |\n        .*?\n        (?:\n            (?:url\\s*\\()|\n            (?:-moz-|-webkit-|-ms-)?\n            (?:auto|default|none|context-menu|help|pointer|progress|\n            wait|cell|crosshair|text|vertical-text|alias|copy|\n            move|no-drop|not-allowed|e-resize|n-resize|ne-resize|\n            nw-resize|s-resize|se-resize|sw-resize|w-resize|\n            ew-resize|ns-resize|nesw-resize|nwse-resize|col-resize|\n            row-resize|all-scroll|zoom-in|zoom-out|grab|grabbing\n            normal|bold|light(er?)|serif|sans|monospace)\n        )\n    ) | (\n        (\n        altGlyph|altGlyphDef|altGlyphItem|animate|animateColor|\n        animateMotion|animateTransform|circle|clipPath|color-profile|\n        defs|desc|ellipse|feBlend|feColorMatrix|\n        feComponentTransfer|feComposite|feConvolveMatrix|\n        feDiffuseLighting|feDisplacementMap|feDistantLight|feFlood|\n        feFuncA|feFuncB|feFuncG|feFuncR|feGaussianBlur|feImage|feMerge|\n        feMergeNode|feMorphology|feOffset|fePointLight|\n        feSpecularLighting|feSpotLight|feTile|feTurbulence|filter|\n        font-face|font-face-format|font-face-name|font-face-src|\n        font-face-uri|foreignObject|g|glyph|glyphRef|hkern|image|line|\n        linearGradient|marker|mask|metadata|missing-glyph|mpath|path|\n        pattern|polygon|polyline|radialGradient|rect|set|stop|svg|\n        switch|symbol|text|textPath|tref|tspan|use|view|vkern|\n        a|abbr|acronym|address|applet|area|article|aside|audio|b|base|\n        basefont|bdi|bdo|bgsound|big|blink|blockquote|body|br|button|\n        canvas|caption|center|cite|code|col|colgroup|data|\n        datalist|dd|decorator|del|details|dfn|dir|div|dl|dt|element|\n        em|embed|fieldset|figcaption|figure|footer|form|frame|\n        frameset|h1|h2|h3|h4|h5|h6|head|header|hgroup|hr|html|i|iframe|\n        img|input|ins|isindex|kbd|keygen|label|legend|li|link|listing|\n        main|map|mark|marquee|menu|menuitem|meta|meter|nav|nobr|\n        noframes|noscript|object|ol|optgroup|option|output|p|param|\n        plaintext|pre|progress|q|rp|rt|ruby|s|samp|script|section|\n        select|shadow|small|source|spacer|span|strike|strong|style|\n        sub|summary|sup|table|tbody|td|template|textarea|tfoot|th|\n        thead|time|title|tr|track|tt|u|ul|var|video|wbr|xmp)\n\n        \\s*([\\s,.#\\[]|:[^\\s]|(?=\\{|(?m:$)))\n\n    ) | (\n        [:~>\\[*\\/]       # symbols but they are valid for selector\n\n    ) | (\n\n        \\+\\s*[\\w$-]+\\b\\s*      # are an identifier starting with (?m:$)\n        (?!\\()              # and they can\'t have anything besides\n\n    ) | (                    # for animtions\n\n        \\d+(\\.\\d+)?%|(from|to)\\b\n\n    ) | (                   # Placeholder selectors\n\n        \\$[\\w$-]+\\b\\s*      # are an identifier starting with (?m:$)\n        (?=(?m:$)|\\{)            # and they can\'t have anything besides\n\n    ) | (                   # CSS class\n\n        \\.[a-zA-Z0-9_-]+\n\n    ) | (                   # CSS id\n\n        \\#[a-zA-Z0-9_-]+\n\n    ) | (                   # Reference to parent\n\n        ([\\w\\d_-]+)?        # matching any word right before &\n        (&)             # & itself, escaped because of plist\n        ([\\w\\d_-]+)?        # matching any word right after &\n    )\n)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9188 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)                # multi-line regex definition mode\n(?<=^|;|{)\\s*    # starts after begining of line, \'{\' or \';\'\'\n(?=                 # lookahead for\n    (\n     [a-zA-Z0-9_-]  # then a letter\n     |              # or\n     (\\{(.*?)\\})    # interpolation\n     |              # or\n     (/\\*.*?\\*/)    # comment\n    )+\n\n    \\s*[:\\s]\\s*     # value is separted by colon or space\n\n    (?!(\\s*\\{))     # if there are only spaces afterwards\n\n    (?!\n        [^}]*?      # checking for an unclosed curly braces on this\n        \\{          # line because if one exists it means that\n        [^}]*       # this is a selector and not a property\n        ((?m:$)|\\})\n    )\n)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9189 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("@extends?\\s"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 52636761252233216,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9191 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9229 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9206 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9213 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9215 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9214 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9222 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9208 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{"),
      scope: vec![
        Scope {
            a: 47288521996435601,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\}"),
      scope: vec![
        Scope {
            a: 47288521948070033,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }