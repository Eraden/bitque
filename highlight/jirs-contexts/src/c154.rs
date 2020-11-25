
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
      regex: Regex::new("\\b(?i:POSIX\\s+path|frontmost|id|name|running|version|days?|weekdays?|months?|years?|time|date\\s+string|time\\s+string|length|rest|reverse|items?|contents|quoted\\s+form|characters?|paragraphs?|words?)\\b"),
      scope: vec![
        Scope {
            a: 61925255106199785,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:activate|log|clipboard\\s+info|set\\s+the\\s+clipboard\\s+to|the\\s+clipboard|info\\s+for|list\\s+(disks|folder)|mount\\s+volume|path\\s+to(\\s+resource)?|close\\s+access|get\\s+eof|open\\s+for\\s+access|read|set\\s+eof|write|open\\s+location|current\\s+date|do\\s+shell\\s+script|get\\s+volume\\s+settings|random\\s+number|round|set\\s+volume|system\\s+(attribute|info)|time\\s+to\\s+GMT|load\\s+script|run\\s+script|scripting\\s+components|store\\s+script|copy|count|get|launch|run|set|ASCII\\s+(character|number)|localized\\s+string|offset|summarize|beep|choose\\s+(application|color|file(\\s+name)?|folder|from\\s+list|remote\\s+application|URL)|delay|display\\s+(alert|dialog)|say)\\b"),
      scope: vec![
        Scope {
            a: 61925255106199868,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:get|run)\\b"),
      scope: vec![
        Scope {
            a: 61925255106199560,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:anything|data|text|upper\\s+case|propert(y|ies))\\b"),
      scope: vec![
        Scope {
            a: 61925366775349256,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:alias|class)(es)?\\b"),
      scope: vec![
        Scope {
            a: 61925366775349256,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:app(lication)?|boolean|character|constant|date|event|file(\\s+specification)?|handler|integer|item|keystroke|linked\\s+list|list|machine|number|picture|preposition|POSIX\\s+file|real|record|reference(\\s+form)?|RGB\\s+color|script|sound|text\\s+item|type\\s+class|vector|writing\\s+code(\\s+info)?|zone|((international|styled(\\s+(Clipboard|Unicode))?|Unicode)\\s+)?text|((C|encoded|Pascal)\\s+)?string)s?\\b"),
      scope: vec![
        Scope {
            a: 61925366775349256,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?ix)\\b\n\t(\t(cubic\\s+(centi)?|square\\s+(kilo)?|centi|kilo)met(er|re)s\n\t|\tsquare\\s+(yards|feet|miles)|cubic\\s+(yards|feet|inches)|miles|inches\n\t|\tlit(re|er)s|gallons|quarts\n\t|\t(kilo)?grams|ounces|pounds\n\t|\tdegrees\\s+(Celsius|Fahrenheit|Kelvin)\n\t)\n\\b"),
      scope: vec![
        Scope {
            a: 61925366775349565,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:seconds|minutes|hours|days)\\b"),
      scope: vec![
        Scope {
            a: 61925366775349556,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }