
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
      regex: Regex::new("\\b(all|and|any|bitand|bitcmp|bitget|bitmax|bitor|bitset|bitshift|bitxor|eq|ge|gt|isa|isappdata|iscell|iscellstr|ischar|iscom|isdir|isempty|isequal|isequalwithequalnans|isevent|isfield|isfinite|isfloat|isglobal|ishandle|ishold|isinf|isinteger|isinterface|isjava|iskeyword|isletter|islogical|ismac|ismember|ismethod|isnan|isnumeric|isobject|ispc|ispref|isprime|isprop|isreal|isscalar|issorted|isspace|issparse|isstrprop|isstruct|isstudent|isunix|isvarname|isvector|le|lt|mislocked|or|ne|not|setxor|union|unique|xor)\\b"),
      scope: vec![
        Scope {
            a: 52636628102086656,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }