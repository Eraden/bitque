
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Terraform".to_string(),
  file_extensions: vec!["tf".to_string(),"tfvars".to_string(),"hcl".to_string()],
  scope: Scope { a: 845064880259072, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("predeclared_funcs".to_string(), "abs|ceil|floor|log|max|min|pow|signum|chomp|format|formatlist|indent|join|lower|regex|regexall|replace|split|strrev|substr|title|trimspace|upper|chunklist|coalesce|coalescelist|compact|concat|contains|distinct|element|flatten|index|keys|length|list|lookup|map|matchkeys|merge|range|reverse|setintersection|setproduct|setunion|slice|sort|transpose|values|zipmap|base64decode|base64encode|base64gzip|csvdecode|jsondecode|jsonencode|urlencode|yamldecode|yamlencode|abspath|dirname|pathexpand|basename|file|fileexists|fileset|filebase64|templatefile|formatdate|timeadd|timestamp|base64sha256|base64sha512|bcrypt|filebase64sha256|filebase64sha512|filemd5|filemd1|filesha256|filesha512|md5|rsadecrypt|sha1|sha256|sha512|uuid|uuidv5|cidrhost|cidrnetmask|cidrsubnet|tobool|tolist|tomap|tonumber|toset|tostring".to_string());
    v.insert("terraform_known_blocks".to_string(), "resource|provider|variable|output|locals|module|data|terraform".to_string());
    v.insert("char_escapes".to_string(), "\\\\[nrt\"\\\\]|\\\\u(\\h{8}|\\h{4})".to_string());
    v.insert("terraform_type_keywords".to_string(), "any|string|number|bool".to_string());
    v.insert("exponent".to_string(), "([Ee][+-]?)".to_string());
    v.insert("identifer".to_string(), "\\b(?!null|false|true)[[:alpha:]][[:alnum:]_-]*\\b".to_string());
    v.insert("named_values".to_string(), "var|local|module|data|path|terraform".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("numeric_literals".to_string(), ContextId { index: 9384 });
    v.insert("__start".to_string(), ContextId { index: 9366 });
    v.insert("#anon_tuple_for_expression_0".to_string(), ContextId { index: 9364 });
    v.insert("#anon_attribute_access_0".to_string(), ContextId { index: 9347 });
    v.insert("#anon_functions_0".to_string(), ContextId { index: 9353 });
    v.insert("comments".to_string(), ContextId { index: 9373 });
    v.insert("#anon_brackets_0".to_string(), ContextId { index: 9352 });
    v.insert("#anon_objects_0".to_string(), ContextId { index: 9359 });
    v.insert("#anon_block_comments_0".to_string(), ContextId { index: 9351 });
    v.insert("object_key_values".to_string(), ContextId { index: 9386 });
    v.insert("string_literals".to_string(), ContextId { index: 9391 });
    v.insert("#anon_string_literals_0".to_string(), ContextId { index: 9363 });
    v.insert("object_for_expression".to_string(), ContextId { index: 9385 });
    v.insert("brackets".to_string(), ContextId { index: 9371 });
    v.insert("expressions".to_string(), ContextId { index: 9374 });
    v.insert("block_comments".to_string(), ContextId { index: 9370 });
    v.insert("for_expression_body".to_string(), ContextId { index: 9375 });
    v.insert("literal_values".to_string(), ContextId { index: 9381 });
    v.insert("__main".to_string(), ContextId { index: 9365 });
    v.insert("string_interpolation".to_string(), ContextId { index: 9390 });
    v.insert("named_value_references".to_string(), ContextId { index: 9383 });
    v.insert("#anon_block_1".to_string(), ContextId { index: 9349 });
    v.insert("#anon_parens_0".to_string(), ContextId { index: 9361 });
    v.insert("#anon_objects_1".to_string(), ContextId { index: 9360 });
    v.insert("#anon_inline_comments_0".to_string(), ContextId { index: 9357 });
    v.insert("attribute_definition".to_string(), ContextId { index: 9368 });
    v.insert("operators".to_string(), ContextId { index: 9388 });
    v.insert("#anon_imports_1".to_string(), ContextId { index: 9356 });
    v.insert("#anon_string_interpolation_0".to_string(), ContextId { index: 9362 });
    v.insert("inline_comments".to_string(), ContextId { index: 9379 });
    v.insert("main".to_string(), ContextId { index: 9382 });
    v.insert("#anon_block_0".to_string(), ContextId { index: 9348 });
    v.insert("block".to_string(), ContextId { index: 9369 });
    v.insert("language_constants".to_string(), ContextId { index: 9380 });
    v.insert("attribute_access".to_string(), ContextId { index: 9367 });
    v.insert("imports".to_string(), ContextId { index: 9378 });
    v.insert("#anon_object_for_expression_0".to_string(), ContextId { index: 9358 });
    v.insert("comma".to_string(), ContextId { index: 9372 });
    v.insert("functions".to_string(), ContextId { index: 9376 });
    v.insert("#anon_heredoc_0".to_string(), ContextId { index: 9354 });
    v.insert("#anon_block_2".to_string(), ContextId { index: 9350 });
    v.insert("heredoc".to_string(), ContextId { index: 9377 });
    v.insert("#anon_imports_0".to_string(), ContextId { index: 9355 });
    v.insert("tuple_for_expression".to_string(), ContextId { index: 9392 });
    v.insert("parens".to_string(), ContextId { index: 9389 });
    v.insert("type_keywords".to_string(), ContextId { index: 9393 });
    v.insert("objects".to_string(), ContextId { index: 9387 });
    v
  }
} }