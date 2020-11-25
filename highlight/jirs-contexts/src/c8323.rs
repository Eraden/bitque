
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
      regex: Regex::new("\\s*(\\*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632122882,
            b: 3940649673949184,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8244 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(&)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628158644347,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8245 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(\\b(a|abbr|acronym|address|applet|area|article|aside|audio|b|base|basefont|bdi|bdo|big|blockquote|body|br|button|canvas|caption|cite|code|col|colgroup|content|data|datalist|dd|del|details|dfn|dir|dialog|div|dl|dt|element|em|embed|eventsource|fieldset|figure|figcaption|footer|form|frame|frameset|h[1-6]|head|header|hgroup|hr|html|i|iframe|img|input|ins|isindex|kbd|keygen|label|legend|li|link|main|map|mark|menu|meta|meter|nav|noframes|noscript|object|ol|optgroup|option|output|p|param|picture|pre|progress|q|rp|rt|rtc|s|samp|script|section|select|shadow|small|source|span|strike|strong|style|sub|summary|sup|svg|table|tbody|td|template|textarea|tfoot|th|thead|time|title|tr|track|tt|u|ul|var|video|wbr|xmp|circle|clipPath|defs|ellipse|filter|foreignObject|g|glyph|glyphRef|image|line|linearGradient|marker|mask|path|pattern|polygon|polyline|radialGradient|rect|stop|switch|symbol|text|textPath|tref|tspan|use)\\b)(?=(?x:\n    (:{1,2})(?:before|after|first-line|first-letter) # CSS1 & CSS2 require : or ::\n  | (::)(-(?:moz|ms|webkit)-)?(?:(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)) # CSS3 requires ::\n)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632122382,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8248 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(\\b(a|abbr|acronym|address|applet|area|article|aside|audio|b|base|basefont|bdi|bdo|big|blockquote|body|br|button|canvas|caption|cite|code|col|colgroup|content|data|datalist|dd|del|details|dfn|dir|dialog|div|dl|dt|element|em|embed|eventsource|fieldset|figure|figcaption|footer|form|frame|frameset|h[1-6]|head|header|hgroup|hr|html|i|iframe|img|input|ins|isindex|kbd|keygen|label|legend|li|link|main|map|mark|menu|meta|meter|nav|noframes|noscript|object|ol|optgroup|option|output|p|param|picture|pre|progress|q|rp|rt|rtc|s|samp|script|section|select|shadow|small|source|span|strike|strong|style|sub|summary|sup|svg|table|tbody|td|template|textarea|tfoot|th|thead|time|title|tr|track|tt|u|ul|var|video|wbr|xmp|circle|clipPath|defs|ellipse|filter|foreignObject|g|glyph|glyphRef|image|line|linearGradient|marker|mask|path|pattern|polygon|polyline|radialGradient|rect|stop|switch|symbol|text|textPath|tref|tspan|use)\\b)(?=:\\b(active|any-link|blank|checked|current|default|defined|disabled|drop|empty|enabled|first|first-child|first-of-type|fullscreen|future|focus|focus-visible|focus-within|host|hover|indeterminate|in-range|invalid|last-child|last-of-type|left|link|local-link|only-child|only-of-type|optional|out-of-range|past|placeholder-shown|read-only|read-write|required|right|root|scope|target|target-within|user-invalid|valid|visited)\\b(?![-]))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632122382,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8249 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(\\b(a|abbr|acronym|address|applet|area|article|aside|audio|b|base|basefont|bdi|bdo|big|blockquote|body|br|button|canvas|caption|cite|code|col|colgroup|content|data|datalist|dd|del|details|dfn|dir|dialog|div|dl|dt|element|em|embed|eventsource|fieldset|figure|figcaption|footer|form|frame|frameset|h[1-6]|head|header|hgroup|hr|html|i|iframe|img|input|ins|isindex|kbd|keygen|label|legend|li|link|main|map|mark|menu|meta|meter|nav|noframes|noscript|object|ol|optgroup|option|output|p|param|picture|pre|progress|q|rp|rt|rtc|s|samp|script|section|select|shadow|small|source|span|strike|strong|style|sub|summary|sup|svg|table|tbody|td|template|textarea|tfoot|th|thead|time|title|tr|track|tt|u|ul|var|video|wbr|xmp|circle|clipPath|defs|ellipse|filter|foreignObject|g|glyph|glyphRef|image|line|linearGradient|marker|mask|path|pattern|polygon|polyline|radialGradient|rect|stop|switch|symbol|text|textPath|tref|tspan|use)\\b)(?=:(\\b(dir|lang|matches|not|has|drop|nth-last-child|nth-child|nth-last-of-type|nth-of-type)\\b)\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632122382,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8250 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(\\b(a|abbr|acronym|address|applet|area|article|aside|audio|b|base|basefont|bdi|bdo|big|blockquote|body|br|button|canvas|caption|cite|code|col|colgroup|content|data|datalist|dd|del|details|dfn|dir|dialog|div|dl|dt|element|em|embed|eventsource|fieldset|figure|figcaption|footer|form|frame|frameset|h[1-6]|head|header|hgroup|hr|html|i|iframe|img|input|ins|isindex|kbd|keygen|label|legend|li|link|main|map|mark|menu|meta|meter|nav|noframes|noscript|object|ol|optgroup|option|output|p|param|picture|pre|progress|q|rp|rt|rtc|s|samp|script|section|select|shadow|small|source|span|strike|strong|style|sub|summary|sup|svg|table|tbody|td|template|textarea|tfoot|th|thead|time|title|tr|track|tt|u|ul|var|video|wbr|xmp|circle|clipPath|defs|ellipse|filter|foreignObject|g|glyph|glyphRef|image|line|linearGradient|marker|mask|path|pattern|polygon|polyline|radialGradient|rect|stop|switch|symbol|text|textPath|tref|tspan|use)\\b)(?=:(-(moz|ms|webkit)-)(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632122382,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8251 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(\\b(a|abbr|acronym|address|applet|area|article|aside|audio|b|base|basefont|bdi|bdo|big|blockquote|body|br|button|canvas|caption|cite|code|col|colgroup|content|data|datalist|dd|del|details|dfn|dir|dialog|div|dl|dt|element|em|embed|eventsource|fieldset|figure|figcaption|footer|form|frame|frameset|h[1-6]|head|header|hgroup|hr|html|i|iframe|img|input|ins|isindex|kbd|keygen|label|legend|li|link|main|map|mark|menu|meta|meter|nav|noframes|noscript|object|ol|optgroup|option|output|p|param|picture|pre|progress|q|rp|rt|rtc|s|samp|script|section|select|shadow|small|source|span|strike|strong|style|sub|summary|sup|svg|table|tbody|td|template|textarea|tfoot|th|thead|time|title|tr|track|tt|u|ul|var|video|wbr|xmp|circle|clipPath|defs|ellipse|filter|foreignObject|g|glyph|glyphRef|image|line|linearGradient|marker|mask|path|pattern|polygon|polyline|radialGradient|rect|stop|switch|symbol|text|textPath|tref|tspan|use)\\b)(?![-:])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632122382,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8252 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(\\.)(?=(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)|@)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392186477183179,
            b: 3940649673949184,
        },
        Scope {
            a: 47288629324873742,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8253 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(\\#)(?=(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)|@)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392186477183489,
            b: 3940649673949184,
        },
        Scope {
            a: 47288629324873742,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8254 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(?=\\[)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8255 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8318 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8317 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*((?:>{1,3}|[~+]))(?![>~+])\\s*"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46445565886464000,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288620754862094,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }