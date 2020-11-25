
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9204 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9211 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9201 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9223 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\$[\\w$-]+\\b"),
      scope: vec![
        Scope {
            a: 59392186480590993,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[:~>]"),
      scope: vec![
        Scope {
            a: 52636628132233361,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x) # multi-line regex definition mode\n\\b(\n    altGlyph|altGlyphDef|altGlyphItem|animate|animateColor|\n    animateMotion|animateTransform|circle|clipPath|color-profile|\n    defs|desc|ellipse|feBlend|feColorMatrix|\n    feComponentTransfer|feComposite|feConvolveMatrix|\n    feDiffuseLighting|feDisplacementMap|feDistantLight|feFlood|\n    feFuncA|feFuncB|feFuncG|feFuncR|feGaussianBlur|feImage|feMerge|\n    feMergeNode|feMorphology|feOffset|fePointLight|\n    feSpecularLighting|feSpotLight|feTile|feTurbulence|filter|\n    font-face|font-face-format|font-face-name|font-face-src|\n    font-face-uri|foreignObject|g|glyph|glyphRef|hkern|image|line|\n    linearGradient|marker|mask|metadata|missing-glyph|mpath|path|\n    pattern|polygon|polyline|radialGradient|rect|set|stop|svg|\n    switch|symbol|text|textPath|tref|tspan|use|view|vkern|\n    a|abbr|acronym|address|applet|area|article|aside|audio|b|base|\n    basefont|bdi|bdo|bgsound|big|blink|blockquote|body|br|button|\n    canvas|caption|center|cite|code|col|colgroup|content|data|\n    datalist|dd|decorator|del|details|dfn|dir|div|dl|dt|element|\n    em|embed|fieldset|figcaption|figure|font|footer|form|frame|\n    frameset|h1|h2|h3|h4|h5|h6|head|header|hgroup|hr|html|i|iframe|\n    img|input|ins|isindex|kbd|keygen|label|legend|li|link|listing|\n    main|map|mark|marquee|menu|menuitem|meta|meter|nav|nobr|\n    noframes|noscript|object|ol|optgroup|option|output|p|param|\n    plaintext|pre|progress|q|rp|rt|ruby|s|samp|script|section|\n    select|shadow|small|source|spacer|span|strike|strong|style|\n    sub|summary|sup|table|tbody|td|template|textarea|tfoot|th|\n    thead|time|title|tr|track|tt|u|ul|var|video|wbr|xmp\n)\\b"),
      scope: vec![
        Scope {
            a: 59392130632122513,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\.[a-zA-Z0-9_-]+"),
      scope: vec![
        Scope {
            a: 59392186477183179,
            b: 40813871623045120,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("#[a-zA-Z0-9_-]+"),
      scope: vec![
        Scope {
            a: 59392186477183489,
            b: 40813871623045120,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)            # multi-line regex definition mode\n([\\w\\d_-]+)?    # matching any word right before &\n(&)         # & itself, escaped because of plist\n([\\w\\d_-]+)?    # matching any word right after &"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392186477183121,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259061531705344,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392186477183121,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }