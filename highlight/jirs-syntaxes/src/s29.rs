
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "HTML".to_string(),
  file_extensions: vec!["html".to_string(),"htm".to_string(),"shtml".to_string(),"xhtml".to_string()],
  scope: Scope { a: 281496453775360, b: 0 },
  first_line_match: Some("(?i)<(!DOCTYPE\\s*)?html".to_string()),
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("custom_element_char".to_string(), "(?x:\n  # https://html.spec.whatwg.org/multipage/custom-elements.html#custom-elements-core-concepts\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n)".to_string());
    v.insert("tag_name".to_string(), "[A-Za-z]{{tag_name_char}}*".to_string());
    v.insert("script_close_lookahead".to_string(), "(?i:(?=(?:-->\\s*)?</script))".to_string());
    v.insert("javascript_mime_type".to_string(), "(?ix:\n  # https://mimesniff.spec.whatwg.org/#javascript-mime-type\n  (?:application|text)/(?:x-)?(?:java|ecma)script\n  | text/javascript1\\.[0-5]\n  | text/jscript\n  | text/livescript\n)".to_string());
    v.insert("tag_name_char".to_string(), "[^{{ascii_space}}/<>]".to_string());
    v.insert("unquoted_attribute_break".to_string(), "(?=[{{ascii_space}}]|/?>)".to_string());
    v.insert("form_tag_name".to_string(), "(?ix:\n  button|datalist|input|label|legend|meter|optgroup|option|output|progress|select|template|textarea\n){{tag_name_break}}".to_string());
    v.insert("ascii_space".to_string(), "\\t\\n\\f ".to_string());
    v.insert("tag_name_break".to_string(), "(?=[^{{tag_name_char}}])".to_string());
    v.insert("unquoted_attribute_start".to_string(), "(?=[^{{ascii_space}}=>])".to_string());
    v.insert("attribute_name_break".to_string(), "(?={{attribute_name_char}})".to_string());
    v.insert("inline_tag_name".to_string(), "(?ix:\n  abbr|acronym|area|audio|b|base|basefont|bdi|bdo|big|br|canvas|caption|cite|code|del|details|dfn|dialog|em|font|head|html|i|img|ins|isindex|kbd|li|link|map|mark|menu|menuitem|meta|noscript|param|picture|q|rp|rt|rtc|ruby|s|samp|script|small|source|span|strike|strong|style|sub|summary|sup|time|title|track|tt|u|var|video|wbr\n){{tag_name_break}}".to_string());
    v.insert("attribute_name_start".to_string(), "(?=[^{{attribute_name_char}}])".to_string());
    v.insert("block_tag_name".to_string(), "(?ix:\n  address|applet|article|aside|blockquote|center|dd|dir|div|dl|dt|figcaption|figure|footer|frame|frameset|h1|h2|h3|h4|h5|h6|header|iframe|menu|nav|noframes|object|ol|p|pre|section|ul\n){{tag_name_break}}".to_string());
    v.insert("attribute_name_char".to_string(), "[{{ascii_space}}=/>]".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("doctype-content".to_string(), ContextId { index: 2100 });
    v.insert("#anon_tag-attribute-value-separator_0".to_string(), ContextId { index: 2077 });
    v.insert("script-common".to_string(), ContextId { index: 2109 });
    v.insert("#anon_script-javascript_0".to_string(), ContextId { index: 2063 });
    v.insert("#anon_tag-generic-attribute-value_0".to_string(), ContextId { index: 2085 });
    v.insert("#anon_main_6".to_string(), ContextId { index: 2057 });
    v.insert("#anon_tag-generic-attribute-value_2".to_string(), ContextId { index: 2087 });
    v.insert("__start".to_string(), ContextId { index: 2096 });
    v.insert("script-javascript".to_string(), ContextId { index: 2111 });
    v.insert("string-single-quoted".to_string(), ContextId { index: 2116 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 2051 });
    v.insert("entities".to_string(), ContextId { index: 2105 });
    v.insert("#anon_style-css_0".to_string(), ContextId { index: 2071 });
    v.insert("#anon_main_7".to_string(), ContextId { index: 2058 });
    v.insert("#anon_tag-event-attribute-value_3".to_string(), ContextId { index: 2084 });
    v.insert("#anon_tag-generic-attribute-value_1".to_string(), ContextId { index: 2086 });
    v.insert("#anon_tag-style-attribute-value_1".to_string(), ContextId { index: 2092 });
    v.insert("#anon_doctype-content_0".to_string(), ContextId { index: 2050 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 2052 });
    v.insert("style-css".to_string(), ContextId { index: 2119 });
    v.insert("style-other".to_string(), ContextId { index: 2120 });
    v.insert("#anon_script-javascript_1".to_string(), ContextId { index: 2064 });
    v.insert("#anon_cdata_0".to_string(), ContextId { index: 2048 });
    v.insert("#anon_style-css_1".to_string(), ContextId { index: 2072 });
    v.insert("style-type-decider".to_string(), ContextId { index: 2122 });
    v.insert("tag-event-attribute-value".to_string(), ContextId { index: 2137 });
    v.insert("tag-generic-attribute".to_string(), ContextId { index: 2138 });
    v.insert("tag-id-attribute-value".to_string(), ContextId { index: 2146 });
    v.insert("#anon_script-other_0".to_string(), ContextId { index: 2066 });
    v.insert("string-double-quoted".to_string(), ContextId { index: 2115 });
    v.insert("tag-custom-body".to_string(), ContextId { index: 2129 });
    v.insert("tag-generic-attribute-equals".to_string(), ContextId { index: 2139 });
    v.insert("#anon_tag-event-attribute-value_1".to_string(), ContextId { index: 2082 });
    v.insert("doctype-meta".to_string(), ContextId { index: 2102 });
    v.insert("tag-style-attribute-equals".to_string(), ContextId { index: 2151 });
    v.insert("doctype".to_string(), ContextId { index: 2099 });
    v.insert("tag-class-attribute-meta".to_string(), ContextId { index: 2127 });
    v.insert("tag-end-maybe-self-closing".to_string(), ContextId { index: 2132 });
    v.insert("tag-generic-attribute-name".to_string(), ContextId { index: 2141 });
    v.insert("#anon_tag-event-attribute-value_2".to_string(), ContextId { index: 2083 });
    v.insert("tag-style-attribute-meta".to_string(), ContextId { index: 2152 });
    v.insert("#anon_tag-id-attribute-value_0".to_string(), ContextId { index: 2088 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 2053 });
    v.insert("doctype-name".to_string(), ContextId { index: 2103 });
    v.insert("tag-event-attribute-meta".to_string(), ContextId { index: 2136 });
    v.insert("style-type-attribute".to_string(), ContextId { index: 2121 });
    v.insert("#anon_style-other_0".to_string(), ContextId { index: 2074 });
    v.insert("tag-other-body".to_string(), ContextId { index: 2147 });
    v.insert("tag-custom-name".to_string(), ContextId { index: 2130 });
    v.insert("#anon_comment_0".to_string(), ContextId { index: 2049 });
    v.insert("#anon_script-html_0".to_string(), ContextId { index: 2062 });
    v.insert("#anon_script-type-attribute_0".to_string(), ContextId { index: 2067 });
    v.insert("#anon_script-type-attribute_1".to_string(), ContextId { index: 2068 });
    v.insert("#anon_string-double-quoted_0".to_string(), ContextId { index: 2069 });
    v.insert("#anon_tag-event-attribute-value_0".to_string(), ContextId { index: 2081 });
    v.insert("#anon_script-close-tag_0".to_string(), ContextId { index: 2060 });
    v.insert("doctype-content-type".to_string(), ContextId { index: 2101 });
    v.insert("script-type-decider".to_string(), ContextId { index: 2114 });
    v.insert("tag-event-attribute-equals".to_string(), ContextId { index: 2135 });
    v.insert("tag-generic-attribute-value".to_string(), ContextId { index: 2142 });
    v.insert("tag-other-name".to_string(), ContextId { index: 2148 });
    v.insert("#anon_string-single-quoted_0".to_string(), ContextId { index: 2070 });
    v.insert("#anon_tag-class-attribute-value_2".to_string(), ContextId { index: 2080 });
    v.insert("#anon_style-css_2".to_string(), ContextId { index: 2073 });
    v.insert("script-html".to_string(), ContextId { index: 2110 });
    v.insert("tag-class-attribute".to_string(), ContextId { index: 2125 });
    v.insert("#anon_tag-style-attribute-value_0".to_string(), ContextId { index: 2091 });
    v.insert("cdata".to_string(), ContextId { index: 2097 });
    v.insert("else-pop".to_string(), ContextId { index: 2104 });
    v.insert("style-common".to_string(), ContextId { index: 2118 });
    v.insert("tag-class-attribute-value".to_string(), ContextId { index: 2128 });
    v.insert("tag-end".to_string(), ContextId { index: 2131 });
    v.insert("tag-end-self-closing".to_string(), ContextId { index: 2133 });
    v.insert("#anon_tag-class-attribute-value_1".to_string(), ContextId { index: 2079 });
    v.insert("tag-style-attribute-value".to_string(), ContextId { index: 2153 });
    v.insert("#anon_script-javascript_2".to_string(), ContextId { index: 2065 });
    v.insert("script-type-attribute".to_string(), ContextId { index: 2113 });
    v.insert("main".to_string(), ContextId { index: 2107 });
    v.insert("#anon_style-type-attribute_0".to_string(), ContextId { index: 2075 });
    v.insert("#anon_tag-id-attribute-value_2".to_string(), ContextId { index: 2090 });
    v.insert("tag-attribute-value-separator".to_string(), ContextId { index: 2123 });
    v.insert("tag-id-attribute-meta".to_string(), ContextId { index: 2145 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 2054 });
    v.insert("#anon_tag-class-attribute-value_0".to_string(), ContextId { index: 2078 });
    v.insert("#anon_tag-style-attribute-value_2".to_string(), ContextId { index: 2093 });
    v.insert("script-other".to_string(), ContextId { index: 2112 });
    v.insert("#anon_tag-id-attribute-value_1".to_string(), ContextId { index: 2089 });
    v.insert("tag-id-attribute-equals".to_string(), ContextId { index: 2144 });
    v.insert("#anon_main_8".to_string(), ContextId { index: 2059 });
    v.insert("#anon_script-close-tag_1".to_string(), ContextId { index: 2061 });
    v.insert("tag-id-attribute".to_string(), ContextId { index: 2143 });
    v.insert("#anon_main_5".to_string(), ContextId { index: 2056 });
    v.insert("__main".to_string(), ContextId { index: 2095 });
    v.insert("tag-generic-attribute-meta".to_string(), ContextId { index: 2140 });
    v.insert("tag-style-attribute".to_string(), ContextId { index: 2150 });
    v.insert("tag-class-attribute-equals".to_string(), ContextId { index: 2126 });
    v.insert("#anon_style-type-attribute_1".to_string(), ContextId { index: 2076 });
    v.insert("#anon_tag-style-attribute-value_3".to_string(), ContextId { index: 2094 });
    v.insert("tag-stuff".to_string(), ContextId { index: 2149 });
    v.insert("tag-attributes".to_string(), ContextId { index: 2124 });
    v.insert("style-close-tag".to_string(), ContextId { index: 2117 });
    v.insert("tag-event-attribute".to_string(), ContextId { index: 2134 });
    v.insert("immediately-pop".to_string(), ContextId { index: 2106 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 2055 });
    v.insert("script-close-tag".to_string(), ContextId { index: 2108 });
    v.insert("comment".to_string(), ContextId { index: 2098 });
    v
  }
} }