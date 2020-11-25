
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
      regex: Regex::new("\\b(addOptional|addParamValue|addRequired|addtodate|arrayfun|assert|blanks|builtin|calendar|cell|celldisp|cellfun|cellplot|clock|cputime|createCopy|datatipinfo|date|datenum|datestr|datevec|dbmex|deal|deblank|depdir|depfun|echo|eomday|error|etime|eval|evalc|evalin|exist|feval|fieldnames|findstr|func2str|genvarname|getfield|global|inferiorto|inmem|intersect|intwarning|lasterr|lasterror|lastwarn|loadobj|lower|methods|methodsview|mex|mexext|mfilename|mlock|munlock|nargchk|nargoutchk|now|orderfields|parse|pcode|regexp|regexpi|regexprep|regexptranslate|rmfield|run|saveobj|setdiff|setfield|sprintf|sscanf|strcat|strcmp|strcmpi|strfind|strings|strjust|strmatch|strncmp|strncmpi|strread|strrep|strtok|strtrim|structfun|strvcat|subsasgn|subsindex|subsref|substruct|superiorto|swapbytes|symvar|tic|timer|timerfind|timerfindall|toc|typecast|upper|warning|weekday|who|whos)\\b"),
      scope: vec![
        Scope {
            a: 52636787015876608,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }