
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Nix".to_string(),
  file_extensions: vec!["nix".to_string()],
  scope: Scope { a: 844974685945856, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_function-parameter_0".to_string(), ContextId { index: 8461 });
    v.insert("#anon_function-definition_4".to_string(), ContextId { index: 8450 });
    v.insert("#anon_string_1".to_string(), ContextId { index: 8481 });
    v.insert("#anon_attrset-or-function_1".to_string(), ContextId { index: 8427 });
    v.insert("function-parameter-default".to_string(), ContextId { index: 8518 });
    v.insert("#anon_parameter-name-and-cont_0".to_string(), ContextId { index: 8476 });
    v.insert("#anon_string_5".to_string(), ContextId { index: 8485 });
    v.insert("attribute-bind".to_string(), ContextId { index: 8489 });
    v.insert("comment-remark".to_string(), ContextId { index: 8501 });
    v.insert("#anon_if_0".to_string(), ContextId { index: 8463 });
    v.insert("#anon_let_5".to_string(), ContextId { index: 8473 });
    v.insert("function-body".to_string(), ContextId { index: 8505 });
    v.insert("attrset-or-function".to_string(), ContextId { index: 8498 });
    v.insert("#anon_function-header-until-colon-with-arg_0".to_string(), ContextId { index: 8459 });
    v.insert("#anon_expression-cont_0".to_string(), ContextId { index: 8440 });
    v.insert("#anon_function-definition_3".to_string(), ContextId { index: 8449 });
    v.insert("#anon_let_1".to_string(), ContextId { index: 8469 });
    v.insert("#anon_attrset-or-function_3".to_string(), ContextId { index: 8429 });
    v.insert("comment".to_string(), ContextId { index: 8500 });
    v.insert("operator-unary".to_string(), ContextId { index: 8526 });
    v.insert("with-assert".to_string(), ContextId { index: 8535 });
    v.insert("#anon_function-definition-brace-opened_0".to_string(), ContextId { index: 8443 });
    v.insert("#anon_attrset-definition_0".to_string(), ContextId { index: 8420 });
    v.insert("attrset-for-sure".to_string(), ContextId { index: 8497 });
    v.insert("parameter-name".to_string(), ContextId { index: 8528 });
    v.insert("#anon_function-definition-brace-opened_1".to_string(), ContextId { index: 8444 });
    v.insert("#anon_function-header-terminal-arg_1".to_string(), ContextId { index: 8456 });
    v.insert("#anon_function-parameter-default_0".to_string(), ContextId { index: 8460 });
    v.insert("attrset-definition".to_string(), ContextId { index: 8495 });
    v.insert("#anon_attrset-definition_1".to_string(), ContextId { index: 8421 });
    v.insert("attribute-bind-from-equals".to_string(), ContextId { index: 8490 });
    v.insert("function-header-open-brace".to_string(), ContextId { index: 8513 });
    v.insert("whitespace".to_string(), ContextId { index: 8534 });
    v.insert("#anon_function-for-sure_0".to_string(), ContextId { index: 8451 });
    v.insert("function-header-close-brace-with-arg".to_string(), ContextId { index: 8512 });
    v.insert("function-contents".to_string(), ContextId { index: 8507 });
    v.insert("parameter-name-and-cont".to_string(), ContextId { index: 8529 });
    v.insert("function-parameter".to_string(), ContextId { index: 8517 });
    v.insert("#anon_attribute-inherit_2".to_string(), ContextId { index: 8416 });
    v.insert("#anon_attrset-for-sure_1".to_string(), ContextId { index: 8424 });
    v.insert("#anon_attrset-or-function_6".to_string(), ContextId { index: 8432 });
    v.insert("#anon_parens-and-cont_0".to_string(), ContextId { index: 8477 });
    v.insert("attrset-definition-brace-opened".to_string(), ContextId { index: 8496 });
    v.insert("#anon_attrset-for-sure_0".to_string(), ContextId { index: 8423 });
    v.insert("function-header-until-colon-no-arg".to_string(), ContextId { index: 8515 });
    v.insert("#anon_attribute-inherit_0".to_string(), ContextId { index: 8414 });
    v.insert("attribute-name".to_string(), ContextId { index: 8492 });
    v.insert("#anon_comment_0".to_string(), ContextId { index: 8435 });
    v.insert("#anon_attribute-bind-from-equals_0".to_string(), ContextId { index: 8413 });
    v.insert("#anon_attribute-inherit_1".to_string(), ContextId { index: 8415 });
    v.insert("#anon_function-definition-brace-opened_2".to_string(), ContextId { index: 8445 });
    v.insert("attrset-contents".to_string(), ContextId { index: 8494 });
    v.insert("main".to_string(), ContextId { index: 8525 });
    v.insert("#anon_attribute-inherit_3".to_string(), ContextId { index: 8417 });
    v.insert("#anon_function-definition_0".to_string(), ContextId { index: 8446 });
    v.insert("if".to_string(), ContextId { index: 8519 });
    v.insert("string-quoted".to_string(), ContextId { index: 8533 });
    v.insert("expression-cont".to_string(), ContextId { index: 8504 });
    v.insert("#anon_list-and-cont_0".to_string(), ContextId { index: 8474 });
    v.insert("#anon_function-header-terminal-arg_0".to_string(), ContextId { index: 8455 });
    v.insert("#anon_string_4".to_string(), ContextId { index: 8484 });
    v.insert("#anon_attrset-definition-brace-opened_0".to_string(), ContextId { index: 8418 });
    v.insert("#anon_attrset-or-function_2".to_string(), ContextId { index: 8428 });
    v.insert("#anon_function-parameter_1".to_string(), ContextId { index: 8462 });
    v.insert("#anon_let_0".to_string(), ContextId { index: 8468 });
    v.insert("#anon_comment_1".to_string(), ContextId { index: 8436 });
    v.insert("parens".to_string(), ContextId { index: 8530 });
    v.insert("function-header-close-brace-no-arg".to_string(), ContextId { index: 8511 });
    v.insert("#anon_function-body_0".to_string(), ContextId { index: 8442 });
    v.insert("#anon_function-header-until-colon-no-arg_0".to_string(), ContextId { index: 8458 });
    v.insert("__start".to_string(), ContextId { index: 8488 });
    v.insert("#anon_attrset-for-sure_2".to_string(), ContextId { index: 8425 });
    v.insert("#anon_attrset-or-function_7".to_string(), ContextId { index: 8433 });
    v.insert("#anon_function-header-open-brace_0".to_string(), ContextId { index: 8454 });
    v.insert("let".to_string(), ContextId { index: 8522 });
    v.insert("#anon_function-body-from-colon_0".to_string(), ContextId { index: 8441 });
    v.insert("function-definition".to_string(), ContextId { index: 8508 });
    v.insert("#anon_function-definition_2".to_string(), ContextId { index: 8448 });
    v.insert("function-for-sure".to_string(), ContextId { index: 8510 });
    v.insert("attribute-inherit".to_string(), ContextId { index: 8491 });
    v.insert("#anon_function-header-terminal-arg_2".to_string(), ContextId { index: 8457 });
    v.insert("function-header-terminal-arg".to_string(), ContextId { index: 8514 });
    v.insert("#anon_string_0".to_string(), ContextId { index: 8480 });
    v.insert("parens-and-cont".to_string(), ContextId { index: 8531 });
    v.insert("#anon_if_3".to_string(), ContextId { index: 8466 });
    v.insert("#anon_attrset-or-function_0".to_string(), ContextId { index: 8426 });
    v.insert("#anon_constants_0".to_string(), ContextId { index: 8437 });
    v.insert("#anon_string_2".to_string(), ContextId { index: 8482 });
    v.insert("illegal".to_string(), ContextId { index: 8520 });
    v.insert("#anon_with-assert_0".to_string(), ContextId { index: 8486 });
    v.insert("__main".to_string(), ContextId { index: 8487 });
    v.insert("#anon_if_1".to_string(), ContextId { index: 8464 });
    v.insert("#anon_attrset-or-function_8".to_string(), ContextId { index: 8434 });
    v.insert("#anon_attrset-or-function_4".to_string(), ContextId { index: 8430 });
    v.insert("#anon_list_0".to_string(), ContextId { index: 8475 });
    v.insert("constants".to_string(), ContextId { index: 8502 });
    v.insert("#anon_attrset-definition-brace-opened_1".to_string(), ContextId { index: 8419 });
    v.insert("#anon_function-definition_1".to_string(), ContextId { index: 8447 });
    v.insert("function-body-from-colon".to_string(), ContextId { index: 8506 });
    v.insert("#anon_let_2".to_string(), ContextId { index: 8470 });
    v.insert("others".to_string(), ContextId { index: 8527 });
    v.insert("function-header-until-colon-with-arg".to_string(), ContextId { index: 8516 });
    v.insert("expression".to_string(), ContextId { index: 8503 });
    v.insert("#anon_constants_1".to_string(), ContextId { index: 8438 });
    v.insert("#anon_function-header-close-brace-with-arg_0".to_string(), ContextId { index: 8453 });
    v.insert("#anon_parens_0".to_string(), ContextId { index: 8478 });
    v.insert("interpolation".to_string(), ContextId { index: 8521 });
    v.insert("function-definition-brace-opened".to_string(), ContextId { index: 8509 });
    v.insert("#anon_attrset-definition_2".to_string(), ContextId { index: 8422 });
    v.insert("list-and-cont".to_string(), ContextId { index: 8524 });
    v.insert("string".to_string(), ContextId { index: 8532 });
    v.insert("#anon_constants_2".to_string(), ContextId { index: 8439 });
    v.insert("#anon_interpolation_0".to_string(), ContextId { index: 8467 });
    v.insert("attribute-name-single".to_string(), ContextId { index: 8493 });
    v.insert("#anon_let_4".to_string(), ContextId { index: 8472 });
    v.insert("list".to_string(), ContextId { index: 8523 });
    v.insert("#anon_if_2".to_string(), ContextId { index: 8465 });
    v.insert("#anon_let_3".to_string(), ContextId { index: 8471 });
    v.insert("#anon_string_3".to_string(), ContextId { index: 8483 });
    v.insert("#anon_attrset-or-function_5".to_string(), ContextId { index: 8431 });
    v.insert("#anon_function-header-close-brace-no-arg_0".to_string(), ContextId { index: 8452 });
    v.insert("bad-reserved".to_string(), ContextId { index: 8499 });
    v.insert("#anon_string-quoted_0".to_string(), ContextId { index: 8479 });
    v
  }
} }