
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
  prototype: Some(
    ContextId {
        index: 5884,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(?x:\n    ((?:null|Null|NULL|~))\n  | ((?x:\n   y|Y|yes|Yes|YES|n|N|no|No|NO\n  |true|True|TRUE|false|False|FALSE\n  |on|On|ON|off|Off|OFF\n))\n  | ((?x:\n    ([-+]? (0b) [0-1_]+) # (base 2)\n  | ([-+]? (0)  [0-7_]+) # (base 8)\n  | ([-+]? (?: 0|[1-9][0-9_]*)) # (base 10)\n  | ([-+]? (0x) [0-9a-fA-F_]+) # (base 16)\n  | ([-+]? [1-9] [0-9_]* (?: :[0-5]?[0-9])+) # (base 60)\n))\n  | ((?x:\n    ([-+]? (?: [0-9] [0-9_]*)? (\\.) [0-9.]* (?: [eE] [-+] [0-9]+)?) # (base 10)\n  | ([-+]? [0-9] [0-9_]* (?: :[0-5]?[0-9])+ (\\.) [0-9_]*) # (base 60)\n  | ([-+]? (\\.) (?: inf|Inf|INF)) # (infinity)\n  | (      (\\.) (?: nan|NaN|NAN)) # (not a number)\n))\n  | ((?x:\n    \\d{4} (-) \\d{2} (-) \\d{2}       # (y-m-d)\n  | \\d{4}                           # (year)\n    (-) \\d{1,2}                     # (month)\n    (-) \\d{1,2}                     # (day)\n    (?: [Tt] | [ \\t]+) \\d{1,2}      # (hour)\n    (:) \\d{2}                       # (minute)\n    (:) \\d{2}                       # (second)\n    (?: (\\.)\\d*)?                   # (fraction)\n    [ \\t]*\n    (?:\n      Z | [-+] \\d{1,2} (?: (:)\\d{1,2})?\n    )?                              # (time zone)\n))\n  | (=)\n  | (<<)\n )\n(?x:\n  (?=\n      \\s* (?m:$)\n    | \\s+ \\#\n    | \\s* : (\\s|(?m:$))\n    | \\s* : [\\[\\]{},]\n    | \\s* [\\[\\]{},]\n  )\n)\n"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955110657261649,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955110657196113,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59955089176461741,
            b: 22799473113563136,
        },
    ]),(5, vec![
        Scope {
            a: 47288629325070764,
            b: 22799473113563136,
        },
    ]),(6, vec![
        Scope {
            a: 59955089176461666,
            b: 22799473113563136,
        },
    ]),(7, vec![
        Scope {
            a: 47288629325070764,
            b: 22799473113563136,
        },
    ]),(8, vec![
        Scope {
            a: 59955089176461530,
            b: 22799473113563136,
        },
    ]),(9, vec![
        Scope {
            a: 59955089176461528,
            b: 22799473113563136,
        },
    ]),(10, vec![
        Scope {
            a: 47288629325070764,
            b: 22799473113563136,
        },
    ]),(11, vec![
        Scope {
            a: 59955089176461537,
            b: 22799473113563136,
        },
    ]),(13, vec![
        Scope {
            a: 59955089176592602,
            b: 22799473113563136,
        },
    ]),(14, vec![
        Scope {
            a: 47288620735397969,
            b: 0,
        },
    ]),(15, vec![
        Scope {
            a: 59955089176592609,
            b: 22799473113563136,
        },
    ]),(16, vec![
        Scope {
            a: 47288620735397969,
            b: 0,
        },
    ]),(17, vec![
        Scope {
            a: 59955089176592609,
            b: 22799473113563136,
        },
    ]),(18, vec![
        Scope {
            a: 47288620735397969,
            b: 0,
        },
    ]),(19, vec![
        Scope {
            a: 59955089176592609,
            b: 22799473113563136,
        },
    ]),(20, vec![
        Scope {
            a: 47288620735397969,
            b: 0,
        },
    ]),(21, vec![
        Scope {
            a: 59955136450265169,
            b: 0,
        },
    ]),(22, vec![
        Scope {
            a: 47288620764299345,
            b: 0,
        },
    ]),(23, vec![
        Scope {
            a: 47288620764299345,
            b: 0,
        },
    ]),(24, vec![
        Scope {
            a: 47288620764299345,
            b: 0,
        },
    ]),(25, vec![
        Scope {
            a: 47288620764299345,
            b: 0,
        },
    ]),(26, vec![
        Scope {
            a: 47288620741296209,
            b: 0,
        },
    ]),(27, vec![
        Scope {
            a: 47288620741296209,
            b: 0,
        },
    ]),(28, vec![
        Scope {
            a: 47288620741296209,
            b: 0,
        },
    ]),(29, vec![
        Scope {
            a: 47288620741296209,
            b: 0,
        },
    ]),(30, vec![
        Scope {
            a: 59955110661259345,
            b: 0,
        },
    ]),(31, vec![
        Scope {
            a: 59955110679674961,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }