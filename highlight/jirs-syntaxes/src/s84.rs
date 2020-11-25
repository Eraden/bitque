
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "XML".to_string(),
  file_extensions: vec!["xml".to_string(),"xsd".to_string(),"xslt".to_string(),"tld".to_string(),"dtml".to_string(),"rng".to_string(),"rss".to_string(),"opml".to_string(),"svg".to_string()],
  scope: Scope { a: 281818574094336, b: 0 },
  first_line_match: Some("(?x:\n  ^(?:\n      <\\?xml\\s\n    | \\s*<([\\w-]+):Envelope\\s+xmlns:\\1\\s*=\\s*\"http://schemas.xmlsoap.org/soap/envelope/\"\\s*>\n    | \\s*(?i:<!DOCTYPE\\s+(?!html[ \\t\\n\\f>]))\n  )\n)".to_string()),
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("qualified_dtd_name".to_string(), "{{name}}(?=[{{dtd_break}}])".to_string());
    v.insert("name".to_string(), "[[:alpha:]:_][[:alnum:]:_.-]*".to_string());
    v.insert("qualified_attribute_name".to_string(), "(?x)\n(?:\n  (?:\n    ({{identifier}})           # 1: valid namespace\n    |\n    ([^:=/<>\\s]+)              # 2: invalid namespace\n  )(:)\n)?                             # namespace is optional\n(?:\n  ({{identifier}})             # 3: valid localname\n  |\n  ([^=/<>\\s]+?)                # 4: invalid localname\n)(?=[=<>\\s]|[/?]>)".to_string());
    v.insert("qualified_tag_name".to_string(), "(?x)\n(?:\n  (?:\n    ({{identifier}})           # 1: valid namespace\n    |\n    ([^?!/<>\\s][^:/<>\\s]*)     # 2: invalid namespace\n  )(:)\n)?                             # namespace is optional\n(?:\n  ({{identifier}})(?=[/<>\\s])  # 3: valid localname\n  |\n  ([^?!/<>\\s][^/<>\\s]*)        # 4: invalid localname\n)".to_string());
    v.insert("dtd_break".to_string(), "[\'\"\\[\\]()<>\\s]".to_string());
    v.insert("invalid_dtd_name".to_string(), "[^{{dtd_break}}]+".to_string());
    v.insert("identifier".to_string(), "[[:alpha:]_][[:alnum:]_.-]*".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("string-double-quoted".to_string(), ContextId { index: 5823 });
    v.insert("dtd-content-type".to_string(), ContextId { index: 5795 });
    v.insert("dtd-entity".to_string(), ContextId { index: 5804 });
    v.insert("dtd-common-name".to_string(), ContextId { index: 5793 });
    v.insert("tag-attribute-separator-key-value".to_string(), ContextId { index: 5835 });
    v.insert("#anon_comment_0".to_string(), ContextId { index: 5771 });
    v.insert("tag-end-missing-pop".to_string(), ContextId { index: 5837 });
    v.insert("#anon_tag-attribute-separator-key-value_0".to_string(), ContextId { index: 5778 });
    v.insert("dtd-entity-content".to_string(), ContextId { index: 5805 });
    v.insert("preprocessor-end".to_string(), ContextId { index: 5821 });
    v.insert("dtd-attlist-parens".to_string(), ContextId { index: 5792 });
    v.insert("dtd-subset-meta".to_string(), ContextId { index: 5814 });
    v.insert("string-single-quoted".to_string(), ContextId { index: 5828 });
    v.insert("tag-attribute".to_string(), ContextId { index: 5834 });
    v.insert("main".to_string(), ContextId { index: 5819 });
    v.insert("#anon_dtd-attlist-parens_0".to_string(), ContextId { index: 5772 });
    v.insert("dtd-subset-brackets".to_string(), ContextId { index: 5813 });
    v.insert("dtd-attlist-meta".to_string(), ContextId { index: 5791 });
    v.insert("tag".to_string(), ContextId { index: 5833 });
    v.insert("#anon_dtd-subset-brackets_0".to_string(), ContextId { index: 5774 });
    v.insert("dtd-else-pop".to_string(), ContextId { index: 5802 });
    v.insert("string-double-quoted-body".to_string(), ContextId { index: 5824 });
    v.insert("dtd-element-content".to_string(), ContextId { index: 5798 });
    v.insert("dtd-content-quoted".to_string(), ContextId { index: 5794 });
    v.insert("#anon_tag_1".to_string(), ContextId { index: 5780 });
    v.insert("dtd-element-parens".to_string(), ContextId { index: 5801 });
    v.insert("doctype-root-name".to_string(), ContextId { index: 5787 });
    v.insert("dtd-element-name".to_string(), ContextId { index: 5800 });
    v.insert("doctype-meta".to_string(), ContextId { index: 5786 });
    v.insert("dtd-attlist".to_string(), ContextId { index: 5789 });
    v.insert("dtd-notation-meta".to_string(), ContextId { index: 5810 });
    v.insert("#anon_dtd-element-parens_0".to_string(), ContextId { index: 5773 });
    v.insert("dtd-end".to_string(), ContextId { index: 5803 });
    v.insert("string-quoted".to_string(), ContextId { index: 5826 });
    v.insert("__start".to_string(), ContextId { index: 5782 });
    v.insert("doctype".to_string(), ContextId { index: 5785 });
    v.insert("dtd-notation-name".to_string(), ContextId { index: 5811 });
    v.insert("tag-else-pop".to_string(), ContextId { index: 5836 });
    v.insert("#anon_preprocessor_1".to_string(), ContextId { index: 5777 });
    v.insert("dtd-element".to_string(), ContextId { index: 5797 });
    v.insert("__main".to_string(), ContextId { index: 5781 });
    v.insert("dtd-subset".to_string(), ContextId { index: 5812 });
    v.insert("dtd-content-unquoted".to_string(), ContextId { index: 5796 });
    v.insert("dtd-unknown".to_string(), ContextId { index: 5816 });
    v.insert("#anon_dtd-unknown_0".to_string(), ContextId { index: 5775 });
    v.insert("string-double-quoted-pop".to_string(), ContextId { index: 5825 });
    v.insert("cdata".to_string(), ContextId { index: 5783 });
    v.insert("#anon_cdata_0".to_string(), ContextId { index: 5770 });
    v.insert("preprocessor".to_string(), ContextId { index: 5820 });
    v.insert("should-be-entity".to_string(), ContextId { index: 5822 });
    v.insert("dtd-subset-name".to_string(), ContextId { index: 5815 });
    v.insert("#anon_preprocessor_0".to_string(), ContextId { index: 5776 });
    v.insert("dtd".to_string(), ContextId { index: 5788 });
    v.insert("comment".to_string(), ContextId { index: 5784 });
    v.insert("dtd-entity-meta".to_string(), ContextId { index: 5806 });
    v.insert("dtd-entity-name".to_string(), ContextId { index: 5807 });
    v.insert("entity".to_string(), ContextId { index: 5817 });
    v.insert("entity-parameter".to_string(), ContextId { index: 5818 });
    v.insert("dtd-notation".to_string(), ContextId { index: 5809 });
    v.insert("dtd-element-meta".to_string(), ContextId { index: 5799 });
    v.insert("string-quoted-pop".to_string(), ContextId { index: 5827 });
    v.insert("string-single-quoted-body".to_string(), ContextId { index: 5829 });
    v.insert("string-single-quoted-pop".to_string(), ContextId { index: 5830 });
    v.insert("string-unquoted-pop".to_string(), ContextId { index: 5832 });
    v.insert("string-unquoted".to_string(), ContextId { index: 5831 });
    v.insert("dtd-attlist-content".to_string(), ContextId { index: 5790 });
    v.insert("dtd-entity-punctuation".to_string(), ContextId { index: 5808 });
    v.insert("#anon_tag_0".to_string(), ContextId { index: 5779 });
    v
  }
} }