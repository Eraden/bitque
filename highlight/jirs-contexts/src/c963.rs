
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
      regex: Regex::new("(?=[/@{)])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b([a-z](?:(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n))*-(?:(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n))*)\\b"),
      scope: vec![
        Scope {
            a: 59392130632122880,
            b: 3940649673949184,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(a|abbr|acronym|address|applet|area|article|aside|audio|b|base|basefont|bdi|bdo|big|blockquote|body|br|button|canvas|caption|cite|code|col|colgroup|content|data|datalist|dd|del|details|dfn|dir|dialog|div|dl|dt|element|em|embed|eventsource|fieldset|figure|figcaption|footer|form|frame|frameset|h[1-6]|head|header|hgroup|hr|html|i|iframe|img|input|ins|isindex|kbd|keygen|label|legend|li|link|main|map|mark|menu|meta|meter|nav|noframes|noscript|object|ol|optgroup|option|output|p|param|picture|pre|progress|q|rp|rt|rtc|s|samp|script|section|select|shadow|small|source|span|strike|strong|style|sub|summary|sup|svg|table|tbody|td|template|textarea|tfoot|th|thead|time|title|tr|track|tt|u|ul|var|video|wbr|xmp|circle|clipPath|defs|ellipse|filter|foreignObject|g|glyph|glyphRef|image|line|linearGradient|marker|mask|path|pattern|polygon|polyline|radialGradient|rect|stop|switch|symbol|text|textPath|tref|tspan|use)\\b"),
      scope: vec![
        Scope {
            a: 59392130632122382,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\.)(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)"),
      scope: vec![
        Scope {
            a: 59392186477183179,
            b: 3940649673949184,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324873742,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(#)(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)"),
      scope: vec![
        Scope {
            a: 59392186477183489,
            b: 3940649673949184,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324873742,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*"),
      scope: vec![
        Scope {
            a: 59392130632122882,
            b: 3940649673949184,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:>{1,3}|[~+]))(?![>~+])"),
      scope: vec![
        Scope {
            a: 47288620754862094,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:>{1,3}|[~+])){2,}"),
      scope: vec![
        Scope {
            a: 50103314687393806,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1028 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1027 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\["),
      scope: vec![
        Scope {
            a: 47288629324873742,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 964 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 997 })),
]
} }