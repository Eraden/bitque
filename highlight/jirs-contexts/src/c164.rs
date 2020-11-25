
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
      regex: Regex::new("\\b(audio (data|file))\\b"),
      scope: vec![
        Scope {
            a: 61925366773449039,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(alias(es)?|(Classic|local|network|system|user) domain objects?|disk( item)?s?|domains?|file( package)?s?|folders?|items?)\\b"),
      scope: vec![
        Scope {
            a: 61925366773449040,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(delete|open|move)\\b"),
      scope: vec![
        Scope {
            a: 61925255104299344,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(folder actions?|scripts?)\\b"),
      scope: vec![
        Scope {
            a: 61925366773449038,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(attach action to|attached scripts|edit action of|remove action from)\\b"),
      scope: vec![
        Scope {
            a: 61925255104299342,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(movie data|movie file)\\b"),
      scope: vec![
        Scope {
            a: 61925366773449041,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(log out|restart|shut down|sleep)\\b"),
      scope: vec![
        Scope {
            a: 61925255104299346,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(((application |desk accessory )?process|(check|combo )?box)(es)?|(action|attribute|browser|(busy|progress|relevance) indicator|color well|column|drawer|group|grow area|image|incrementor|list|menu( bar)?( item)?|(menu |pop up |radio )?button|outline|(radio|tab|splitter) group|row|scroll (area|bar)|sheet|slider|splitter|static text|table|text (area|field)|tool bar|UI element|window)s?)\\b"),
      scope: vec![
        Scope {
            a: 61925366773449043,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(click|key code|keystroke|perform|select)\\b"),
      scope: vec![
        Scope {
            a: 61925255104299347,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(property list (file|item))\\b"),
      scope: vec![
        Scope {
            a: 61925366773449044,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(annotation|QuickTime (data|file)|track)s?\\b"),
      scope: vec![
        Scope {
            a: 61925366773449045,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((abort|begin|end) transaction)\\b"),
      scope: vec![
        Scope {
            a: 61925255104299294,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(XML (attribute|data|element|file)s?)\\b"),
      scope: vec![
        Scope {
            a: 61925366773448784,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(print settings|users?|login items?)\\b"),
      scope: vec![
        Scope {
            a: 61925366773448929,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }