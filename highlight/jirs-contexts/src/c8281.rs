
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
      regex: Regex::new("\\s*(\\b([a-z](?:(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n))*-(?:(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n))*)\\b(?!(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)))((?x:\n    (:{1,2})(?:before|after|first-line|first-letter) # CSS1 & CSS2 require : or ::\n  | (::)(-(?:moz|ms|webkit)-)?(?:(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)) # CSS3 requires ::\n)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632122880,
            b: 3940649673949184,
        },
    ]),(3, vec![
        Scope {
            a: 59392186489110542,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629324873742,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629324873742,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 61925375377735694,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8172 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(\\b([a-z](?:(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n))*-(?:(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n))*)\\b(?!(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)))((:)(\\b(active|any-link|blank|checked|current|default|defined|disabled|drop|empty|enabled|first|first-child|first-of-type|fullscreen|future|focus|focus-visible|focus-within|host|hover|indeterminate|in-range|invalid|last-child|last-of-type|left|link|local-link|only-child|only-of-type|optional|out-of-range|past|placeholder-shown|read-only|read-write|required|right|root|scope|target|target-within|user-invalid|valid|visited)\\b(?![-])))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632122880,
            b: 3940649673949184,
        },
    ]),(3, vec![
        Scope {
            a: 59392186486751246,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629324873742,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8173 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(\\b([a-z](?:(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n))*-(?:(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n))*)\\b(?!(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)))((:)(dir|lang))(?=\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632122880,
            b: 3940649673949184,
        },
    ]),(3, vec![
        Scope {
            a: 46444883141918945,
            b: 142426398345134080,
        },
    ]),(4, vec![
        Scope {
            a: 47288629324873742,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8176 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(\\b([a-z](?:(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n))*-(?:(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n))*)\\b(?!(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)))((:)(matches|not|has))(?=\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632122880,
            b: 3940649673949184,
        },
    ]),(3, vec![
        Scope {
            a: 46444882986663936,
            b: 0,
        },
        Scope {
            a: 59392186486751246,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629324873742,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8178 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(\\b([a-z](?:(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n))*-(?:(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n))*)\\b(?!(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)))((:)(drop))(?=\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632122880,
            b: 3940649673949184,
        },
    ]),(3, vec![
        Scope {
            a: 46444882986663936,
            b: 0,
        },
        Scope {
            a: 59392186486751246,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629324873742,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8180 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(\\b([a-z](?:(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n))*-(?:(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n))*)\\b(?!(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)))((:)(nth-last-child|nth-child|nth-last-of-type|nth-of-type))(?=\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632122880,
            b: 3940649673949184,
        },
    ]),(3, vec![
        Scope {
            a: 46444882986663936,
            b: 0,
        },
        Scope {
            a: 59392186486751246,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629324873742,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8182 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(\\b([a-z](?:(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n))*-(?:(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n))*)\\b(?!(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)))((:)(-(moz|ms|webkit)-)(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632122382,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392186486751246,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629324873742,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 61925375377735694,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8174 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(\\b([a-z](?:(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n))*-(?:(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n))*)\\b(?!(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)))(?![-:])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632122880,
            b: 3940649673949184,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8175 }),
    ]),
      with_prototype: None
    }),
]
} }