
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
      regex: Regex::new("((abs|ceil|floor|log|max|min|pow|signum|chomp|format|formatlist|indent|join|lower|regex|regexall|replace|split|strrev|substr|title|trimspace|upper|chunklist|coalesce|coalescelist|compact|concat|contains|distinct|element|flatten|index|keys|length|list|lookup|map|matchkeys|merge|range|reverse|setintersection|setproduct|setunion|slice|sort|transpose|values|zipmap|base64decode|base64encode|base64gzip|csvdecode|jsondecode|jsonencode|urlencode|yamldecode|yamlencode|abspath|dirname|pathexpand|basename|file|fileexists|fileset|filebase64|templatefile|formatdate|timeadd|timestamp|base64sha256|base64sha512|bcrypt|filebase64sha256|filebase64sha512|filemd5|filemd1|filesha256|filesha512|md5|rsadecrypt|sha1|sha256|sha512|uuid|uuidv5|cidrhost|cidrnetmask|cidrsubnet|tobool|tolist|tomap|tonumber|toset|tostring)|\\b(\\b(?!null|false|true)[\\p{L}][\\p{L}\\p{N}_-]*\\b)\\b)(\\()"),
      scope: vec![],
      captures: Some(vec![(2, vec![
        Scope {
            a: 61925255090602133,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49258881143341056,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288521948004534,
            b: 41939771529887744,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9353 }),
    ]),
      with_prototype: None
    }),
]
} }