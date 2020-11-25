
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Vue Component".to_string(),
  file_extensions: vec!["vue".to_string()],
  scope: Scope { a: 281496461639680, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("attribute_char".to_string(), "(?:[^ \"\'>/=\\x00-\\x1f\\x7f-\\x9f])".to_string());
    v.insert("block_tag_name".to_string(), "(?ix)(?:\n  address|applet|article|aside|blockquote|center|dd|dir|div|dl|dt|figcaption|figure|footer|frame|frameset|h1|h2|h3|h4|h5|h6|header|iframe|menu|nav|noframes|object|ol|p|pre|section|ul\n)\\b".to_string());
    v.insert("not_equals_lookahead".to_string(), "(?=\\s*[^\\s=])".to_string());
    v.insert("unquoted_attribute_value".to_string(), "(?:[^\\s<>/\'\'\"]|/(?!>))+".to_string());
    v.insert("form_tag_name".to_string(), "(?ix)(?:\n  button|datalist|input|label|legend|meter|optgroup|option|output|progress|select|template|textarea\n)\\b".to_string());
    v.insert("inline_tag_name".to_string(), "(?ix)(?:\n  abbr|acronym|area|audio|b|base|basefont|bdi|bdo|big|br|canvas|caption|cite|code|del|details|dfn|dialog|em|font|head|html|i|img|ins|isindex|kbd|li|link|map|mark|menu|menuitem|meta|noscript|param|picture|q|rp|rt|rtc|ruby|s|samp|script|small|source|span|strike|strong|style|sub|summary|sup|time|title|track|tt|u|var|video|wbr\n)\\b".to_string());
    v.insert("javascript_mime_type".to_string(), "(?ix)(?:\n  # https://mimesniff.spec.whatwg.org/#javascript-mime-type\n  (?:application|text)/(?:x-)?(?:java|ecma)script\n  | text/javascript1\\.[0-5]\n  | text/jscript\n  | text/livescript\n)".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("script-javascript".to_string(), ContextId { index: 10153 });
    v.insert("tag-id-attribute-equals".to_string(), ContextId { index: 10183 });
    v.insert("#anon_main_6".to_string(), ContextId { index: 10049 });
    v.insert("#anon_script-lang-attribute_0".to_string(), ContextId { index: 10060 });
    v.insert("#anon_script-lang-decider_5".to_string(), ContextId { index: 10069 });
    v.insert("#anon_style-css_1".to_string(), ContextId { index: 10080 });
    v.insert("#anon_tag-event-attribute-value_3".to_string(), ContextId { index: 10116 });
    v.insert("tag-generic-attribute".to_string(), ContextId { index: 10178 });
    v.insert("template-lang-decider".to_string(), ContextId { index: 10194 });
    v.insert("#anon_main_8".to_string(), ContextId { index: 10051 });
    v.insert("#anon_tag-style-attribute-value_0".to_string(), ContextId { index: 10121 });
    v.insert("#anon_main_13".to_string(), ContextId { index: 10042 });
    v.insert("#anon_template-lang-decider_11".to_string(), ContextId { index: 10130 });
    v.insert("#anon_js-string_1".to_string(), ContextId { index: 10034 });
    v.insert("#anon_script-lang-decider_1".to_string(), ContextId { index: 10063 });
    v.insert("#anon_script-lang-decider_2".to_string(), ContextId { index: 10066 });
    v.insert("#anon_script-lang-decider_6".to_string(), ContextId { index: 10070 });
    v.insert("#anon_style-lang-decider_12".to_string(), ContextId { index: 10088 });
    v.insert("#anon_style-lang-decider_23".to_string(), ContextId { index: 10100 });
    v.insert("#anon_style-lang-decider_22".to_string(), ContextId { index: 10099 });
    v.insert("#anon_tag-class-attribute-value_1".to_string(), ContextId { index: 10112 });
    v.insert("#anon_tag-id-attribute-value_1".to_string(), ContextId { index: 10120 });
    v.insert("#anon_template-lang-decider_1".to_string(), ContextId { index: 10128 });
    v.insert("#anon_script-lang-decider_7".to_string(), ContextId { index: 10071 });
    v.insert("#anon_mustache-expression_2".to_string(), ContextId { index: 10055 });
    v.insert("#anon_script-other_0".to_string(), ContextId { index: 10074 });
    v.insert("#anon_style-lang-decider_3".to_string(), ContextId { index: 10101 });
    v.insert("#anon_tag-generic-attribute-value_0".to_string(), ContextId { index: 10117 });
    v.insert("#anon_template-lang-decider_8".to_string(), ContextId { index: 10137 });
    v.insert("#anon_script-lang-decider_4".to_string(), ContextId { index: 10068 });
    v.insert("#anon_template-lang-decider_3".to_string(), ContextId { index: 10132 });
    v.insert("script-close-tag".to_string(), ContextId { index: 10150 });
    v.insert("script-common".to_string(), ContextId { index: 10151 });
    v.insert("script-type-decider".to_string(), ContextId { index: 10158 });
    v.insert("string-double-quoted".to_string(), ContextId { index: 10159 });
    v.insert("style-lang-attribute".to_string(), ContextId { index: 10164 });
    v.insert("style-lang-decider".to_string(), ContextId { index: 10165 });
    v.insert("#anon_script-html_0".to_string(), ContextId { index: 10056 });
    v.insert("style-other".to_string(), ContextId { index: 10166 });
    v.insert("tag-class-attribute".to_string(), ContextId { index: 10170 });
    v.insert("tag-class-attribute-equals".to_string(), ContextId { index: 10171 });
    v.insert("tag-generic-attribute-equals".to_string(), ContextId { index: 10179 });
    v.insert("#anon_template-lang-decider_2".to_string(), ContextId { index: 10131 });
    v.insert("template-common".to_string(), ContextId { index: 10192 });
    v.insert("#anon_style-lang-decider_2".to_string(), ContextId { index: 10096 });
    v.insert("#anon_tag-style-attribute-value_3".to_string(), ContextId { index: 10124 });
    v.insert("immediately-pop".to_string(), ContextId { index: 10145 });
    v.insert("tag-class-attribute-value".to_string(), ContextId { index: 10173 });
    v.insert("#anon_js-string_3".to_string(), ContextId { index: 10036 });
    v.insert("#anon_main_10".to_string(), ContextId { index: 10039 });
    v.insert("#anon_template-lang-decider_6".to_string(), ContextId { index: 10135 });
    v.insert("#anon_script-lang-decider_8".to_string(), ContextId { index: 10072 });
    v.insert("#anon_style-lang-decider_16".to_string(), ContextId { index: 10092 });
    v.insert("js-string".to_string(), ContextId { index: 10147 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 10046 });
    v.insert("#anon_tag-style-attribute-value_2".to_string(), ContextId { index: 10123 });
    v.insert("#anon_mustache-expression_0".to_string(), ContextId { index: 10053 });
    v.insert("script-html".to_string(), ContextId { index: 10152 });
    v.insert("#anon_script-lang-attribute_1".to_string(), ContextId { index: 10061 });
    v.insert("#anon_main_14".to_string(), ContextId { index: 10043 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 10047 });
    v.insert("#anon_style-css_2".to_string(), ContextId { index: 10081 });
    v.insert("#anon_style-lang-decider_4".to_string(), ContextId { index: 10102 });
    v.insert("style-common".to_string(), ContextId { index: 10162 });
    v.insert("tag-event-attribute".to_string(), ContextId { index: 10174 });
    v.insert("tag-generic-attribute-meta".to_string(), ContextId { index: 10180 });
    v.insert("tag-id-attribute-meta".to_string(), ContextId { index: 10184 });
    v.insert("tag-style-attribute-value".to_string(), ContextId { index: 10190 });
    v.insert("#anon_template-lang-decider_7".to_string(), ContextId { index: 10136 });
    v.insert("vue-directive".to_string(), ContextId { index: 10197 });
    v.insert("#anon_script-lang-decider_9".to_string(), ContextId { index: 10073 });
    v.insert("#anon_style-other_0".to_string(), ContextId { index: 10108 });
    v.insert("#anon_style-lang-attribute_1".to_string(), ContextId { index: 10083 });
    v.insert("#anon_main_5".to_string(), ContextId { index: 10048 });
    v.insert("tag-event-attribute-equals".to_string(), ContextId { index: 10175 });
    v.insert("#anon_template-lang-attribute_1".to_string(), ContextId { index: 10126 });
    v.insert("#anon_tag-event-attribute-value_0".to_string(), ContextId { index: 10113 });
    v.insert("template-tag".to_string(), ContextId { index: 10196 });
    v.insert("tag-style-attribute-meta".to_string(), ContextId { index: 10189 });
    v.insert("template-mustache".to_string(), ContextId { index: 10195 });
    v.insert("__start".to_string(), ContextId { index: 10141 });
    v.insert("#anon_main_7".to_string(), ContextId { index: 10050 });
    v.insert("#anon_script-lang-decider_11".to_string(), ContextId { index: 10065 });
    v.insert("#anon_template-lang-decider_5".to_string(), ContextId { index: 10134 });
    v.insert("#anon_style-type-attribute_0".to_string(), ContextId { index: 10109 });
    v.insert("#anon_tag-style-attribute-value_1".to_string(), ContextId { index: 10122 });
    v.insert("#anon_template-lang-attribute_0".to_string(), ContextId { index: 10125 });
    v.insert("#anon_style-lang-decider_21".to_string(), ContextId { index: 10098 });
    v.insert("#anon_script-lang-decider_0".to_string(), ContextId { index: 10062 });
    v.insert("#anon_style-lang-decider_15".to_string(), ContextId { index: 10091 });
    v.insert("script-other".to_string(), ContextId { index: 10156 });
    v.insert("#anon_tag-id-attribute-value_0".to_string(), ContextId { index: 10119 });
    v.insert("tag-class-attribute-meta".to_string(), ContextId { index: 10172 });
    v.insert("#anon_tag-generic-attribute-value_1".to_string(), ContextId { index: 10118 });
    v.insert("#anon_template-lang-decider_10".to_string(), ContextId { index: 10129 });
    v.insert("#anon_style-lang-attribute_0".to_string(), ContextId { index: 10082 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 10045 });
    v.insert("#anon_style-lang-decider_10".to_string(), ContextId { index: 10086 });
    v.insert("#anon_style-lang-decider_20".to_string(), ContextId { index: 10097 });
    v.insert("#anon_style-lang-decider_9".to_string(), ContextId { index: 10107 });
    v.insert("#anon_style-css_0".to_string(), ContextId { index: 10079 });
    v.insert("#anon_tag-class-attribute-value_0".to_string(), ContextId { index: 10111 });
    v.insert("tag-id-attribute".to_string(), ContextId { index: 10182 });
    v.insert("tag-generic-attribute-value".to_string(), ContextId { index: 10181 });
    v.insert("else-pop".to_string(), ContextId { index: 10142 });
    v.insert("#anon_script-type-attribute_1".to_string(), ContextId { index: 10076 });
    v.insert("#anon_style-type-attribute_1".to_string(), ContextId { index: 10110 });
    v.insert("tag-event-attribute-meta".to_string(), ContextId { index: 10176 });
    v.insert("#anon_string-single-quoted_0".to_string(), ContextId { index: 10078 });
    v.insert("generic-attribute-name".to_string(), ContextId { index: 10144 });
    v.insert("#anon_template-lang-decider_4".to_string(), ContextId { index: 10133 });
    v.insert("tag-style-attribute-equals".to_string(), ContextId { index: 10188 });
    v.insert("#anon_style-lang-decider_7".to_string(), ContextId { index: 10105 });
    v.insert("template-lang-attribute".to_string(), ContextId { index: 10193 });
    v.insert("#anon_style-lang-decider_0".to_string(), ContextId { index: 10084 });
    v.insert("__main".to_string(), ContextId { index: 10140 });
    v.insert("#anon_script-javascript_0".to_string(), ContextId { index: 10057 });
    v.insert("#anon_style-lang-decider_11".to_string(), ContextId { index: 10087 });
    v.insert("script-lang-attribute".to_string(), ContextId { index: 10154 });
    v.insert("tag-attributes".to_string(), ContextId { index: 10169 });
    v.insert("#anon_main_12".to_string(), ContextId { index: 10041 });
    v.insert("entities".to_string(), ContextId { index: 10143 });
    v.insert("#anon_string-double-quoted_0".to_string(), ContextId { index: 10077 });
    v.insert("tag-style-attribute".to_string(), ContextId { index: 10187 });
    v.insert("tag-stuff".to_string(), ContextId { index: 10186 });
    v.insert("#anon_script-type-attribute_0".to_string(), ContextId { index: 10075 });
    v.insert("#anon_template-mustache_0".to_string(), ContextId { index: 10139 });
    v.insert("#anon_style-lang-decider_17".to_string(), ContextId { index: 10093 });
    v.insert("js-attribute-value".to_string(), ContextId { index: 10146 });
    v.insert("#anon_main_9".to_string(), ContextId { index: 10052 });
    v.insert("script-type-attribute".to_string(), ContextId { index: 10157 });
    v.insert("template-close-tag".to_string(), ContextId { index: 10191 });
    v.insert("#anon_style-lang-decider_1".to_string(), ContextId { index: 10085 });
    v.insert("script-lang-decider".to_string(), ContextId { index: 10155 });
    v.insert("#anon_script-lang-decider_3".to_string(), ContextId { index: 10067 });
    v.insert("#anon_js-string_2".to_string(), ContextId { index: 10035 });
    v.insert("#anon_script-javascript_2".to_string(), ContextId { index: 10059 });
    v.insert("tag-event-attribute-value".to_string(), ContextId { index: 10177 });
    v.insert("#anon_main_11".to_string(), ContextId { index: 10040 });
    v.insert("#anon_script-javascript_1".to_string(), ContextId { index: 10058 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 10037 });
    v.insert("#anon_mustache-expression_1".to_string(), ContextId { index: 10054 });
    v.insert("#anon_style-lang-decider_5".to_string(), ContextId { index: 10103 });
    v.insert("#anon_style-lang-decider_13".to_string(), ContextId { index: 10089 });
    v.insert("#anon_style-lang-decider_6".to_string(), ContextId { index: 10104 });
    v.insert("main".to_string(), ContextId { index: 10148 });
    v.insert("style-type-attribute".to_string(), ContextId { index: 10167 });
    v.insert("mustache-expression".to_string(), ContextId { index: 10149 });
    v.insert("#anon_main_15".to_string(), ContextId { index: 10044 });
    v.insert("style-css".to_string(), ContextId { index: 10163 });
    v.insert("#anon_style-lang-decider_18".to_string(), ContextId { index: 10094 });
    v.insert("#anon_style-lang-decider_19".to_string(), ContextId { index: 10095 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 10038 });
    v.insert("#anon_script-lang-decider_10".to_string(), ContextId { index: 10064 });
    v.insert("style-close-tag".to_string(), ContextId { index: 10161 });
    v.insert("style-type-decider".to_string(), ContextId { index: 10168 });
    v.insert("#anon_style-lang-decider_8".to_string(), ContextId { index: 10106 });
    v.insert("#anon_tag-event-attribute-value_2".to_string(), ContextId { index: 10115 });
    v.insert("#anon_js-string_0".to_string(), ContextId { index: 10033 });
    v.insert("string-single-quoted".to_string(), ContextId { index: 10160 });
    v.insert("#anon_template-lang-decider_0".to_string(), ContextId { index: 10127 });
    v.insert("tag-id-attribute-value".to_string(), ContextId { index: 10185 });
    v.insert("#anon_style-lang-decider_14".to_string(), ContextId { index: 10090 });
    v.insert("#anon_template-lang-decider_9".to_string(), ContextId { index: 10138 });
    v.insert("#anon_tag-event-attribute-value_1".to_string(), ContextId { index: 10114 });
    v
  }
} }