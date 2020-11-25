
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
      regex: Regex::new("(?<!\\.)\\b(Array|ArrayBuffer|Boolean|DataView|Date|Float32Array|Float64Array|Function|Infinity|Int16Array|Int32Array|Int8Array|JSON|Map|Math|NaN|Number|Object|Promise|Proxy|Reflect|RegExp|Set|String|Symbol|System|TypeError|Uint16Array|Uint32Array|Uint8Array|Uint8ClampedArray|WeakMap|WeakSet)\\b"),
      scope: vec![
        Scope {
            a: 61925366759751723,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)\\b((?>Eval|Range|Reference|Syntax|Type|URI)?Error)\\b"),
      scope: vec![
        Scope {
            a: 61925366772334635,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?>Buffer)\\b"),
      scope: vec![
        Scope {
            a: 61925366801498155,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }