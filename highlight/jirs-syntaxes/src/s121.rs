
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "GraphQL".to_string(),
  file_extensions: vec!["graphql".to_string(),"gql".to_string()],
  scope: Scope { a: 844914556403712, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("name_begin".to_string(), "(?:[_A-Za-z])".to_string());
    v.insert("name_break".to_string(), "(?!{{name_continue}})".to_string());
    v.insert("name_continue".to_string(), "(?:[_0-9A-Za-z])".to_string());
    v.insert("fractional_part".to_string(), "(?:\\.\\d*)".to_string());
    v.insert("integer_part".to_string(), "(?:-?(?:0|[1-9]\\d*))".to_string());
    v.insert("exponent_part".to_string(), "(?:[Ee][-+]?\\d*)".to_string());
    v.insert("name".to_string(), "(?:{{name_begin}}{{name_continue}}*{{name_break}})".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_value_3".to_string(), ContextId { index: 7605 });
    v.insert("#anon_type-declarations_0".to_string(), ContextId { index: 7594 });
    v.insert("#anon_value_5".to_string(), ContextId { index: 7607 });
    v.insert("enum-name".to_string(), ContextId { index: 7624 });
    v.insert("value".to_string(), ContextId { index: 7651 });
    v.insert("#anon_type-value_0".to_string(), ContextId { index: 7601 });
    v.insert("fragment-name".to_string(), ContextId { index: 7627 });
    v.insert("else-pop".to_string(), ContextId { index: 7622 });
    v.insert("type-declarations".to_string(), ContextId { index: 7643 });
    v.insert("#anon_schema-definition_1".to_string(), ContextId { index: 7592 });
    v.insert("mutation-name".to_string(), ContextId { index: 7634 });
    v.insert("aliased-field-name".to_string(), ContextId { index: 7613 });
    v.insert("schema-definition".to_string(), ContextId { index: 7638 });
    v.insert("#anon_arguments-definition_0".to_string(), ContextId { index: 7578 });
    v.insert("default-value".to_string(), ContextId { index: 7616 });
    v.insert("#anon_field-arguments_1".to_string(), ContextId { index: 7585 });
    v.insert("#anon_type-declarations_1".to_string(), ContextId { index: 7595 });
    v.insert("enum-definition".to_string(), ContextId { index: 7623 });
    v.insert("type-name".to_string(), ContextId { index: 7646 });
    v.insert("#anon_field-arguments_0".to_string(), ContextId { index: 7584 });
    v.insert("#anon_input-value-definition_0".to_string(), ContextId { index: 7587 });
    v.insert("type-condition".to_string(), ContextId { index: 7642 });
    v.insert("input-definition".to_string(), ContextId { index: 7629 });
    v.insert("input-name".to_string(), ContextId { index: 7630 });
    v.insert("type-value".to_string(), ContextId { index: 7649 });
    v.insert("immediately-pop".to_string(), ContextId { index: 7628 });
    v.insert("#anon_type-definition_1".to_string(), ContextId { index: 7598 });
    v.insert("union-name".to_string(), ContextId { index: 7650 });
    v.insert("interface-name".to_string(), ContextId { index: 7632 });
    v.insert("type-named".to_string(), ContextId { index: 7647 });
    v.insert("#anon_type-definition_0".to_string(), ContextId { index: 7597 });
    v.insert("#anon_variable-definitions_2".to_string(), ContextId { index: 7610 });
    v.insert("directive-definition-name".to_string(), ContextId { index: 7618 });
    v.insert("prototype".to_string(), ContextId { index: 7635 });
    v.insert("#anon_value_2".to_string(), ContextId { index: 7604 });
    v.insert("#anon_arguments_1".to_string(), ContextId { index: 7580 });
    v.insert("#anon_arguments_0".to_string(), ContextId { index: 7579 });
    v.insert("#anon_enum-definition_0".to_string(), ContextId { index: 7583 });
    v.insert("#anon_schema-definition_0".to_string(), ContextId { index: 7591 });
    v.insert("#anon_value_1".to_string(), ContextId { index: 7603 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 7589 });
    v.insert("#anon_value_0".to_string(), ContextId { index: 7602 });
    v.insert("scalar-name".to_string(), ContextId { index: 7637 });
    v.insert("selection-set".to_string(), ContextId { index: 7639 });
    v.insert("subscription-name".to_string(), ContextId { index: 7640 });
    v.insert("input-value-definition".to_string(), ContextId { index: 7631 });
    v.insert("type-non-null".to_string(), ContextId { index: 7648 });
    v.insert("#anon_type-implements_0".to_string(), ContextId { index: 7599 });
    v.insert("__start".to_string(), ContextId { index: 7612 });
    v.insert("#anon_aliased-field-name_0".to_string(), ContextId { index: 7577 });
    v.insert("#anon_variable-definitions_0".to_string(), ContextId { index: 7608 });
    v.insert("description".to_string(), ContextId { index: 7617 });
    v.insert("directive-location".to_string(), ContextId { index: 7619 });
    v.insert("variable-definitions".to_string(), ContextId { index: 7652 });
    v.insert("field-arguments".to_string(), ContextId { index: 7626 });
    v.insert("#anon_prototype_0".to_string(), ContextId { index: 7590 });
    v.insert("type-definition".to_string(), ContextId { index: 7644 });
    v.insert("#anon_type-declarations_2".to_string(), ContextId { index: 7596 });
    v.insert("directives".to_string(), ContextId { index: 7621 });
    v.insert("#anon_variable-definitions_1".to_string(), ContextId { index: 7609 });
    v.insert("type".to_string(), ContextId { index: 7641 });
    v.insert("directive-name".to_string(), ContextId { index: 7620 });
    v.insert("arguments".to_string(), ContextId { index: 7614 });
    v.insert("#anon_description_0".to_string(), ContextId { index: 7581 });
    v.insert("#anon_selection-set_0".to_string(), ContextId { index: 7593 });
    v.insert("executable-declaration".to_string(), ContextId { index: 7625 });
    v.insert("#anon_input-definition_0".to_string(), ContextId { index: 7586 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 7588 });
    v.insert("main".to_string(), ContextId { index: 7633 });
    v.insert("query-name".to_string(), ContextId { index: 7636 });
    v.insert("type-implements".to_string(), ContextId { index: 7645 });
    v.insert("__main".to_string(), ContextId { index: 7611 });
    v.insert("#anon_description_1".to_string(), ContextId { index: 7582 });
    v.insert("arguments-definition".to_string(), ContextId { index: 7615 });
    v.insert("#anon_value_4".to_string(), ContextId { index: 7606 });
    v.insert("#anon_type-implements_1".to_string(), ContextId { index: 7600 });
    v
  }
} }