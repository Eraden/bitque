
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
      regex: Regex::new("&(?![A-Za-z0-9]+;)"),
      scope: vec![
        Scope {
            a: 281496456724480,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\*+(\\([^)]*\\)|\\{[^}]*\\})*(\\s+|(?m:$))"),
      scope: vec![
        Scope {
            a: 114280017427103823,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632450127,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^#+(\\([^)]*\\)|\\{[^}]*\\})*\\s+"),
      scope: vec![
        Scope {
            a: 114280017426907215,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632450127,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n\t\t\t\"\t\t\t\t\t\t\t\t# Start name, etc\n\t\t\t\t(?:\t\t\t\t\t\t\t# Attributes\n\t\t\t\t\t# I swear, this is how the language is defined,\n\t\t\t\t\t# couldnt make it up if I tried.\n\t\t\t\t\t(?:\\([^)]+\\))?(?:\\{[^}]+\\})?(?:\\[[^\\]]+\\])?\n\t\t\t\t\t\t# Class, Style, Lang\n\t\t\t\t  | (?:\\{[^}]+\\})?(?:\\[[^\\]]+\\])?(?:\\([^)]+\\))?\n\t\t\t\t\t\t# Style, Lang, Class\n\t\t\t\t  | (?:\\[[^\\]]+\\])?(?:\\{[^}]+\\})?(?:\\([^)]+\\))?\n\t\t\t\t\t\t# Lang, Style, Class\n\t\t\t\t)?\n\t\t\t\t([^\"]+?)\t\t\t\t\t# Link name\n\t\t\t\t\\s?\t\t\t\t\t\t\t# Optional whitespace\n\t\t\t\t(?:\\(([^)]+?)\\))?\n\t\t\t\":\t\t\t\t\t\t\t\t# End name\n\t\t\t(\\w[-\\w_]*)\t\t\t\t\t\t# Linkref\n\t\t\t(?=[^\\w\\/;]*?(<|\\s|(?m:$)))\t\t\t# Catch closing punctuation\n\t\t\t\t\t\t\t\t\t\t\t#  and end of meta.link"),
      scope: vec![
        Scope {
            a: 46443487132778575,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 55451536781411254,
            b: 22236523160141824,
        },
    ]),(2, vec![
        Scope {
            a: 55451536781411182,
            b: 267401567177539584,
        },
    ]),(3, vec![
        Scope {
            a: 59955136418414619,
            b: 22236523160141824,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n\t\t\t\"\t\t\t\t\t\t\t\t# Start name, etc\n\t\t\t\t(?:\t\t\t\t\t\t\t# Attributes\n\t\t\t\t\t# I swear, this is how the language is defined,\n\t\t\t\t\t# couldnt make it up if I tried.\n\t\t\t\t\t(?:\\([^)]+\\))?(?:\\{[^}]+\\})?(?:\\[[^\\]]+\\])?\n\t\t\t\t\t\t# Class, Style, Lang\n\t\t\t\t  | (?:\\{[^}]+\\})?(?:\\[[^\\]]+\\])?(?:\\([^)]+\\))?\n\t\t\t\t\t\t# Style, Lang, Class\n\t\t\t\t  | (?:\\[[^\\]]+\\])?(?:\\{[^}]+\\})?(?:\\([^)]+\\))?\n\t\t\t\t\t\t# Lang, Style, Class\n\t\t\t\t)?\n\t\t\t\t([^\"]+?)\t\t\t\t\t# Link name\n\t\t\t\t\\s?\t\t\t\t\t\t\t# Optional whitespace\n\t\t\t\t(?:\\(([^)]+?)\\))?\n\t\t\t\":\t\t\t\t\t\t\t\t# End Name\n\t\t\t(\\S*?(?:\\w|\\/|;))\t\t\t\t# URL\n\t\t\t(?=[^\\w\\/;]*?(<|\\s|(?m:$)))\t\t\t# Catch closing punctuation\n\t\t\t\t\t\t\t\t\t\t\t#  and end of meta.link"),
      scope: vec![
        Scope {
            a: 46443487170723919,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 55451536781411254,
            b: 22236523160141824,
        },
    ]),(2, vec![
        Scope {
            a: 55451536781411182,
            b: 267401567177539584,
        },
    ]),(3, vec![
        Scope {
            a: 114280588597985359,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n\t\t\t\\!\t\t\t\t\t\t\t\t\t\t# Open image\n\t\t\t(\\<|\\=|\\>)?\t\t\t\t\t\t\t\t# Optional alignment\n\t\t\t(?:\t\t\t\t\t\t\t\t\t\t# Attributes\n\t\t\t\t# I swear, this is how the language is defined,\n\t\t\t\t# couldnt make it up if I tried.\n\t\t\t\t(?:\\([^)]+\\))?(?:\\{[^}]+\\})?(?:\\[[^\\]]+\\])?\n\t\t\t\t\t# Class, Style, Lang\n\t\t\t  | (?:\\{[^}]+\\})?(?:\\[[^\\]]+\\])?(?:\\([^)]+\\))?\n\t\t\t\t\t# Style, Lang, Class\n\t\t\t  | (?:\\[[^\\]]+\\])?(?:\\{[^}]+\\})?(?:\\([^)]+\\))?\n\t\t\t\t\t# Lang, Style, Class\n\t\t\t)?\n\t\t\t(?:\\.[ ])?            \t\t\t\t\t# Optional\n\t\t\t([^\\s(!]+?)         \t\t\t\t\t# Image URL\n\t\t\t\\s?                \t\t\t\t\t\t# Optional space\n\t\t\t(?:\\(((?:[^\\(\\)]|\\([^\\)]+\\))+?)\\))?   \t# Optional title\n\t\t\t\\!\t\t\t\t\t\t\t\t\t\t# Close image\n\t\t\t(?:\n\t\t\t\t:\n\t\t\t\t(\\S*?(?:\\w|\\/|;))\t\t\t\t\t# URL\n\t\t\t\t(?=[^\\w\\/;]*?(<|\\s|(?m:$)))\t\t\t\t# Catch closing punctuation\n\t\t\t)?"),
      scope: vec![
        Scope {
            a: 46445673309077583,
            b: 0,
        },
    ],
      captures: Some(vec![(2, vec![
        Scope {
            a: 114280588597985816,
            b: 22236523160141824,
        },
    ]),(3, vec![
        Scope {
            a: 55451536781411182,
            b: 22236523160141824,
        },
    ]),(4, vec![
        Scope {
            a: 114280588597985359,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\|(\\([^)]*\\)|\\{[^}]*\\})*(\\\\\\||.)+\\|"),
      scope: vec![
        Scope {
            a: 114279806961649085,
            b: 22236523160141824,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632450127,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\B(\\*\\*)((\\([^)]*\\)|\\{[^}]*\\}|\\[[^]]+\\]){0,3})(\\S.*?\\S|\\S)\\*\\*\\B"),
      scope: vec![
        Scope {
            a: 114281679523086336,
            b: 0,
        },
    ],
      captures: Some(vec![(3, vec![
        Scope {
            a: 59392130632450127,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\B(\\*)((\\([^)]*\\)|\\{[^}]*\\}|\\[[^]]+\\]){0,3})(\\S.*?\\S|\\S)\\*\\B"),
      scope: vec![
        Scope {
            a: 114281679523086336,
            b: 0,
        },
    ],
      captures: Some(vec![(3, vec![
        Scope {
            a: 59392130632450127,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\B-((\\([^)]*\\)|\\{[^}]*\\}|\\[[^]]+\\]){0,3})(\\S.*?\\S|\\S)-\\B"),
      scope: vec![
        Scope {
            a: 114281335925702656,
            b: 0,
        },
    ],
      captures: Some(vec![(2, vec![
        Scope {
            a: 59392130632450127,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\B\\+((\\([^)]*\\)|\\{[^}]*\\}|\\[[^]]+\\]){0,3})(\\S.*?\\S|\\S)\\+\\B"),
      scope: vec![
        Scope {
            a: 114281327335768064,
            b: 0,
        },
    ],
      captures: Some(vec![(2, vec![
        Scope {
            a: 59392130632450127,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\b|\\s)_((\\([^)]*\\)|\\{[^}]*\\}|\\[[^]]+\\]){0,3})(\\S.*?\\S|\\S)_(?:\\b|\\s)"),
      scope: vec![
        Scope {
            a: 114282585761185792,
            b: 0,
        },
    ],
      captures: Some(vec![(2, vec![
        Scope {
            a: 59392130632450127,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\B(\\?\\?)((\\([^)]*\\)|\\{[^}]*\\}|\\[[^]]+\\]){0,3})(\\S.*?\\S|\\S)\\?\\?"),
      scope: vec![
        Scope {
            a: 114282585852346447,
            b: 0,
        },
    ],
      captures: Some(vec![(3, vec![
        Scope {
            a: 59392130632450127,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\B(@)((\\([^)]*\\)|\\{[^}]*\\}|\\[[^]]+\\]){0,3})(\\S.*?\\S|\\S)@"),
      scope: vec![
        Scope {
            a: 114282585852346447,
            b: 0,
        },
    ],
      captures: Some(vec![(3, vec![
        Scope {
            a: 59392130632450127,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\B(\\^)((\\([^)]*\\)|\\{[^}]*\\}|\\[[^]]+\\]){0,3})(\\S.*?\\S|\\S)\\^"),
      scope: vec![
        Scope {
            a: 114282585852346447,
            b: 0,
        },
    ],
      captures: Some(vec![(3, vec![
        Scope {
            a: 59392130632450127,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\B(~)((\\([^)]*\\)|\\{[^}]*\\}|\\[[^]]+\\]){0,3})(\\S.*?\\S|\\S)~"),
      scope: vec![
        Scope {
            a: 114282585852346447,
            b: 0,
        },
    ],
      captures: Some(vec![(3, vec![
        Scope {
            a: 59392130632450127,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\B(%)((\\([^)]*\\)|\\{[^}]*\\}|\\[[^]]+\\]){0,3})(\\S.*?\\S|\\S)%"),
      scope: vec![
        Scope {
            a: 114282585852346447,
            b: 0,
        },
    ],
      captures: Some(vec![(3, vec![
        Scope {
            a: 59392130632450127,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\[[0-9+]\\]"),
      scope: vec![
        Scope {
            a: 59392130632122447,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }