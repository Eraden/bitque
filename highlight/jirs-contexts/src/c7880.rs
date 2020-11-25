
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
        a: 844940326207488,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844940326207488,
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
      regex: Regex::new("\\b(\\d+([Ee][+-]?\\d+)?)\\b"),
      scope: vec![
        Scope {
            a: 59955089170235392,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b\\d+[.]\\d*([Ee][+-]?\\d+)?\\b"),
      scope: vec![
        Scope {
            a: 59955089170235392,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[.]\\d+([Ee][+-]?\\d+)?\\b"),
      scope: vec![
        Scope {
            a: 59955089170235392,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bstd[.](abs|acos|asciiLower|asciiUpper|asin|assertEqual|atan|base64|base64Decode|base64DecodeBytes)\\b"),
      scope: vec![
        Scope {
            a: 61925255093420032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bstd[.](ceil|char|codepoint|cos|count|endsWith)\\b"),
      scope: vec![
        Scope {
            a: 61925255093420032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bstd[.](escapeStringBash|escapeStringDollars|escapeStringJson|escapeStringPython)\\b"),
      scope: vec![
        Scope {
            a: 61925255093420032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bstd[.](exp|exponent|extVar|filter|filterMap|flattenArrays|floor|foldl|foldr|format)\\b"),
      scope: vec![
        Scope {
            a: 61925255093420032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bstd[.](isArray|isBoolean|isFunction|isNumber|isObject|isString|join|length|lines|log|makeArray)\\b"),
      scope: vec![
        Scope {
            a: 61925255093420032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bstd[.](manifestIni|manifestJsonEx|manifestPython|manifestPythonVars|manifestXmlJsonml|manifestYamlDoc|manifestYamlStream)\\b"),
      scope: vec![
        Scope {
            a: 61925255093420032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bstd[.](mantissa|map|mapWithIndex|mapWithKey|max|md5|mergePatch|min|mod)\\b"),
      scope: vec![
        Scope {
            a: 61925255093420032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bstd[.](objectFields|objectFieldsAll|objectHas|objectHasAll|objectHasEx|parseHex|parseInt|parseJson|parseOctal|pow|prune|range)\\b"),
      scope: vec![
        Scope {
            a: 61925255093420032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bstd[.](set|setDiff|setInter|setMember|setUnion|sign|sin|sort|split|splitLimit|sqrt)\\b"),
      scope: vec![
        Scope {
            a: 61925255093420032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bstd[.](startsWith|stringChars|strReplace|substr|tan|toString|type|uniq)\\b"),
      scope: vec![
        Scope {
            a: 61925255093420032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bstd[.](native|thisFile|decodeUTF8|deepJoin|encodeUTF8|find|nativeExt|trace)\\b"),
      scope: vec![
        Scope {
            a: 61925255093420032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[$]\\b"),
      scope: vec![
        Scope {
            a: 49259061530066944,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[a-zA-Z_][a-z0-9A-Z_]*\\s*(\\([^)]*\\))?\\s*\\+?::?:?"),
      scope: vec![
        Scope {
            a: 59392130630615160,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(import|importstr)\\b"),
      scope: vec![
        Scope {
            a: 48414576470392832,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(function)\\b"),
      scope: vec![
        Scope {
            a: 52636787020398592,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(self|super)\\b"),
      scope: vec![
        Scope {
            a: 49259061530066944,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(if|then|else|for|in)\\b"),
      scope: vec![
        Scope {
            a: 52636636696543232,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(local|tailstrict)\\b"),
      scope: vec![
        Scope {
            a: 52636787020398592,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(true|false|null)\\b"),
      scope: vec![
        Scope {
            a: 59955110645071872,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(error|assert)\\b"),
      scope: vec![
        Scope {
            a: 52636636696543232,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("//.*(?m:$)"),
      scope: vec![
        Scope {
            a: 51510711019896832,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("#.*(?m:$)"),
      scope: vec![
        Scope {
            a: 51510878523621376,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("/\\*"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7879 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\|\\|\\|"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7883 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7882 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\'"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7881 }),
    ]),
      with_prototype: None
    }),
]
} }