
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "JavaScript (Babel)".to_string(),
  file_extensions: vec!["js".to_string(),"mjs".to_string(),"jsx".to_string(),"babel".to_string(),"es6".to_string(),"cjs".to_string()],
  scope: Scope { a: 844609613725696, b: 0 },
  first_line_match: Some("^#!\\s*/.*\\b(node|js)$\\n?".to_string()),
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_function-declaration-parameters_1".to_string(), ContextId { index: 7752 });
    v.insert("#anon_literal-method_0".to_string(), ContextId { index: 7778 });
    v.insert("#anon_literal-arrow-function_0".to_string(), ContextId { index: 7762 });
    v.insert("function-declaration-parameters".to_string(), ContextId { index: 7812 });
    v.insert("#anon_jsx-tag-end_0".to_string(), ContextId { index: 7759 });
    v.insert("literal-jsx".to_string(), ContextId { index: 7830 });
    v.insert("#anon_function-declaration-parameters_0".to_string(), ContextId { index: 7751 });
    v.insert("#anon_flowtype-tokens_0".to_string(), ContextId { index: 7746 });
    v.insert("literal-constructor".to_string(), ContextId { index: 7825 });
    v.insert("literal-operators".to_string(), ContextId { index: 7840 });
    v.insert("class-method-storage".to_string(), ContextId { index: 7799 });
    v.insert("jsx-attributes".to_string(), ContextId { index: 7816 });
    v.insert("literal-arrow-function".to_string(), ContextId { index: 7822 });
    v.insert("#anon_flowtype-declaration_0".to_string(), ContextId { index: 7743 });
    v.insert("literal-class".to_string(), ContextId { index: 7824 });
    v.insert("literal-number".to_string(), ContextId { index: 7839 });
    v.insert("literal-punctuation".to_string(), ContextId { index: 7842 });
    v.insert("literal-template-string".to_string(), ContextId { index: 7846 });
    v.insert("#anon_class-method-definition_0".to_string(), ContextId { index: 7733 });
    v.insert("#anon_flowtype-polymorph_0".to_string(), ContextId { index: 7745 });
    v.insert("#anon_literal-for_0".to_string(), ContextId { index: 7770 });
    v.insert("#anon_flowtype-declaration_1".to_string(), ContextId { index: 7744 });
    v.insert("literal-function-labels".to_string(), ContextId { index: 7829 });
    v.insert("literal-keywords".to_string(), ContextId { index: 7832 });
    v.insert("comments".to_string(), ContextId { index: 7801 });
    v.insert("literal-switch".to_string(), ContextId { index: 7845 });
    v.insert("#anon_literal-regexp_0".to_string(), ContextId { index: 7780 });
    v.insert("#anon_round-brackets_0".to_string(), ContextId { index: 7787 });
    v.insert("#anon_literal-template-string_1".to_string(), ContextId { index: 7786 });
    v.insert("literal-prototype".to_string(), ContextId { index: 7841 });
    v.insert("support-class".to_string(), ContextId { index: 7854 });
    v.insert("#anon_styled-components_1".to_string(), ContextId { index: 7790 });
    v.insert("jsx-string-quoted".to_string(), ContextId { index: 7819 });
    v.insert("#anon_literal-constructor_0".to_string(), ContextId { index: 7769 });
    v.insert("#anon_literal-function_0".to_string(), ContextId { index: 7773 });
    v.insert("literal-language-constant".to_string(), ContextId { index: 7834 });
    v.insert("literal-function-call".to_string(), ContextId { index: 7828 });
    v.insert("jsx-tag-end".to_string(), ContextId { index: 7820 });
    v.insert("#anon_graphql_2".to_string(), ContextId { index: 7755 });
    v.insert("#anon_jsx-string-quoted_0".to_string(), ContextId { index: 7758 });
    v.insert("core".to_string(), ContextId { index: 7802 });
    v.insert("#anon_graphql_1".to_string(), ContextId { index: 7754 });
    v.insert("#anon_literal-function_1".to_string(), ContextId { index: 7774 });
    v.insert("flowtype-polymorph".to_string(), ContextId { index: 7810 });
    v.insert("flowtype-brackets".to_string(), ContextId { index: 7807 });
    v.insert("#anon_literal-arrow-function-labels_0".to_string(), ContextId { index: 7761 });
    v.insert("#anon_support-other_0".to_string(), ContextId { index: 7794 });
    v.insert("#anon_flowtype-tokens_3".to_string(), ContextId { index: 7749 });
    v.insert("#anon_literal-switch_2".to_string(), ContextId { index: 7784 });
    v.insert("es7-decorators".to_string(), ContextId { index: 7804 });
    v.insert("#anon_literal-template-string_0".to_string(), ContextId { index: 7785 });
    v.insert("#anon_literal-function-labels_0".to_string(), ContextId { index: 7772 });
    v.insert("literal-for".to_string(), ContextId { index: 7826 });
    v.insert("literal-module".to_string(), ContextId { index: 7838 });
    v.insert("#anon_literal-class_3".to_string(), ContextId { index: 7768 });
    v.insert("literal-function".to_string(), ContextId { index: 7827 });
    v.insert("#anon_literal-arrow-function_2".to_string(), ContextId { index: 7764 });
    v.insert("#anon_flowtype-brackets_0".to_string(), ContextId { index: 7742 });
    v.insert("#anon_class-method-definition_1".to_string(), ContextId { index: 7734 });
    v.insert("#anon_jsx-evaluated-code_0".to_string(), ContextId { index: 7757 });
    v.insert("#anon_comments_1".to_string(), ContextId { index: 7739 });
    v.insert("#anon_graphql_0".to_string(), ContextId { index: 7753 });
    v.insert("#anon_literal-class_2".to_string(), ContextId { index: 7767 });
    v.insert("literal-keyword-storage".to_string(), ContextId { index: 7831 });
    v.insert("expression".to_string(), ContextId { index: 7805 });
    v.insert("#anon_flowtype-tokens_4".to_string(), ContextId { index: 7750 });
    v.insert("#anon_literal-class_1".to_string(), ContextId { index: 7766 });
    v.insert("__start".to_string(), ContextId { index: 7796 });
    v.insert("class-method-definition".to_string(), ContextId { index: 7798 });
    v.insert("#anon_styled-components_2".to_string(), ContextId { index: 7791 });
    v.insert("flowtype-identifier".to_string(), ContextId { index: 7809 });
    v.insert("#anon_literal-labels_0".to_string(), ContextId { index: 7777 });
    v.insert("literal-variable".to_string(), ContextId { index: 7847 });
    v.insert("#anon_jsx-tag-start_0".to_string(), ContextId { index: 7760 });
    v.insert("square-brackets".to_string(), ContextId { index: 7851 });
    v.insert("#anon_literal-jsx_0".to_string(), ContextId { index: 7776 });
    v.insert("#anon_literal-arrow-function_1".to_string(), ContextId { index: 7763 });
    v.insert("#anon_flowtype-tokens_2".to_string(), ContextId { index: 7748 });
    v.insert("jsx-attribute-name".to_string(), ContextId { index: 7815 });
    v.insert("#anon_curly-brackets_0".to_string(), ContextId { index: 7740 });
    v.insert("#anon_literal-switch_1".to_string(), ContextId { index: 7783 });
    v.insert("curly-brackets".to_string(), ContextId { index: 7803 });
    v.insert("#anon_class-method-definition_3".to_string(), ContextId { index: 7736 });
    v.insert("__main".to_string(), ContextId { index: 7795 });
    v.insert("flowtype-tokens".to_string(), ContextId { index: 7811 });
    v.insert("flowtype-declaration".to_string(), ContextId { index: 7808 });
    v.insert("string-content".to_string(), ContextId { index: 7852 });
    v.insert("#anon_styled-components_4".to_string(), ContextId { index: 7793 });
    v.insert("#anon_literal-class_0".to_string(), ContextId { index: 7765 });
    v.insert("literal-language-variable".to_string(), ContextId { index: 7835 });
    v.insert("class-properties".to_string(), ContextId { index: 7800 });
    v.insert("jsx-attribute-assignment".to_string(), ContextId { index: 7814 });
    v.insert("support-other".to_string(), ContextId { index: 7855 });
    v.insert("merge-conflits".to_string(), ContextId { index: 7849 });
    v.insert("#anon_class-properties_0".to_string(), ContextId { index: 7737 });
    v.insert("#anon_flowtype-annotation_0".to_string(), ContextId { index: 7741 });
    v.insert("literal-arrow-function-labels".to_string(), ContextId { index: 7823 });
    v.insert("round-brackets".to_string(), ContextId { index: 7850 });
    v.insert("#anon_flowtype-tokens_1".to_string(), ContextId { index: 7747 });
    v.insert("#anon_literal-for_1".to_string(), ContextId { index: 7771 });
    v.insert("#anon_literal-string_0".to_string(), ContextId { index: 7781 });
    v.insert("literal-regexp".to_string(), ContextId { index: 7843 });
    v.insert("literal-string".to_string(), ContextId { index: 7844 });
    v.insert("#anon_class-method-definition_2".to_string(), ContextId { index: 7735 });
    v.insert("#anon_graphql_3".to_string(), ContextId { index: 7756 });
    v.insert("#anon_literal-function_2".to_string(), ContextId { index: 7775 });
    v.insert("flowtype-annotation".to_string(), ContextId { index: 7806 });
    v.insert("jsx-entities".to_string(), ContextId { index: 7817 });
    v.insert("#anon_styled-components_0".to_string(), ContextId { index: 7789 });
    v.insert("literal-method-call".to_string(), ContextId { index: 7837 });
    v.insert("brackets".to_string(), ContextId { index: 7797 });
    v.insert("jsx-evaluated-code".to_string(), ContextId { index: 7818 });
    v.insert("#anon_literal-switch_0".to_string(), ContextId { index: 7782 });
    v.insert("#anon_literal-method_1".to_string(), ContextId { index: 7779 });
    v.insert("#anon_square-brackets_0".to_string(), ContextId { index: 7788 });
    v.insert("graphql".to_string(), ContextId { index: 7813 });
    v.insert("jsx-tag-start".to_string(), ContextId { index: 7821 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 7738 });
    v.insert("literal-method".to_string(), ContextId { index: 7836 });
    v.insert("main".to_string(), ContextId { index: 7848 });
    v.insert("literal-labels".to_string(), ContextId { index: 7833 });
    v.insert("styled-components".to_string(), ContextId { index: 7853 });
    v.insert("#anon_styled-components_3".to_string(), ContextId { index: 7792 });
    v
  }
} }