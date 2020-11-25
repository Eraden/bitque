
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "SCSS".to_string(),
  file_extensions: vec!["scss".to_string()],
  scope: Scope { a: 845026225553408, b: 0 },
  first_line_match: None,
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("functional_pseudo_classes".to_string(), "\\b(dir|lang|matches|not|has|drop|nth-last-child|nth-child|nth-last-of-type|nth-of-type)\\b".to_string());
    v.insert("nmstart".to_string(), "(?:[[_a-zA-Z]{{nonascii}}]|{{escape}})".to_string());
    v.insert("custom_element_chars".to_string(), "(?x:\n    [-_a-z0-9\\x{00B7}]\n  | \\\\\\.\n  | [\\x{00C0}-\\x{00D6}]\n  | [\\x{00D8}-\\x{00F6}]\n  | [\\x{00F8}-\\x{02FF}]\n  | [\\x{0300}-\\x{037D}]\n  | [\\x{037F}-\\x{1FFF}]\n  | [\\x{200C}-\\x{200D}]\n  | [\\x{203F}-\\x{2040}]\n  | [\\x{2070}-\\x{218F}]\n  | [\\x{2C00}-\\x{2FEF}]\n  | [\\x{3001}-\\x{D7FF}]\n  | [\\x{F900}-\\x{FDCF}]\n  | [\\x{FDF0}-\\x{FFFD}]\n  | [\\x{10000}-\\x{EFFFF}]\n)".to_string());
    v.insert("combinators".to_string(), "(?:>{1,3}|[~+])".to_string());
    v.insert("custom_elements".to_string(), "\\b([a-z](?:{{custom_element_chars}})*-(?:{{custom_element_chars}})*)\\b(?!{{ident}})".to_string());
    v.insert("ident".to_string(), "(?:--{{nmchar}}+|-?{{nmstart}}{{nmchar}}*)".to_string());
    v.insert("nmchar".to_string(), "(?:[[-\\w]{{nonascii}}]|{{escape}})".to_string());
    v.insert("escape".to_string(), "(?:{{unicode}}|\\\\[^\\n\\f\\h])".to_string());
    v.insert("pseudo_elements".to_string(), "(?x:\n    (:{1,2})(?:before|after|first-line|first-letter) # CSS1 & CSS2 require : or ::\n  | (::)(-(?:moz|ms|webkit)-)?(?:{{ident}}) # CSS3 requires ::\n)\\b".to_string());
    v.insert("element_names".to_string(), "\\b(a|abbr|acronym|address|applet|area|article|aside|audio|b|base|basefont|bdi|bdo|big|blockquote|body|br|button|canvas|caption|cite|code|col|colgroup|content|data|datalist|dd|del|details|dfn|dir|dialog|div|dl|dt|element|em|embed|eventsource|fieldset|figure|figcaption|footer|form|frame|frameset|h[1-6]|head|header|hgroup|hr|html|i|iframe|img|input|ins|isindex|kbd|keygen|label|legend|li|link|main|map|mark|menu|meta|meter|nav|noframes|noscript|object|ol|optgroup|option|output|p|param|picture|pre|progress|q|rp|rt|rtc|s|samp|script|section|select|shadow|small|source|span|strike|strong|style|sub|summary|sup|svg|table|tbody|td|template|textarea|tfoot|th|thead|time|title|tr|track|tt|u|ul|var|video|wbr|xmp|circle|clipPath|defs|ellipse|filter|foreignObject|g|glyph|glyphRef|image|line|linearGradient|marker|mask|path|pattern|polygon|polyline|radialGradient|rect|stop|switch|symbol|text|textPath|tref|tspan|use)\\b".to_string());
    v.insert("unicode".to_string(), "\\\\\\h{1,6}[ \\t\\n\\f]?".to_string());
    v.insert("regular_pseudo_classes".to_string(), "\\b(active|any-link|blank|checked|current|default|defined|disabled|drop|empty|enabled|first|first-child|first-of-type|fullscreen|future|focus|focus-visible|focus-within|host|hover|indeterminate|in-range|invalid|last-child|last-of-type|left|link|local-link|only-child|only-of-type|optional|out-of-range|past|placeholder-shown|read-only|read-write|required|right|root|scope|target|target-within|user-invalid|valid|visited)\\b(?![-])".to_string());
    v.insert("nonascii".to_string(), "[\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_at-rules_10".to_string(), ContextId { index: 8844 });
    v.insert("media-query".to_string(), ContextId { index: 8890 });
    v.insert("#anon_at-rules_19".to_string(), ContextId { index: 8853 });
    v.insert("#anon_property-value-wrapper_0".to_string(), ContextId { index: 8882 });
    v.insert("curly-braces".to_string(), ContextId { index: 8888 });
    v.insert("main".to_string(), ContextId { index: 8889 });
    v.insert("#anon_at-rules_26".to_string(), ContextId { index: 8861 });
    v.insert("#anon_at-rules_22".to_string(), ContextId { index: 8857 });
    v.insert("#anon_at-rules_6".to_string(), ContextId { index: 8867 });
    v.insert("#anon_properties_0".to_string(), ContextId { index: 8874 });
    v.insert("#anon_at-rules_11".to_string(), ContextId { index: 8845 });
    v.insert("#anon_at-rules_20".to_string(), ContextId { index: 8855 });
    v.insert("at-rule-punctuation".to_string(), ContextId { index: 8885 });
    v.insert("#anon_at-rules_3".to_string(), ContextId { index: 8864 });
    v.insert("#anon_at-rules_7".to_string(), ContextId { index: 8868 });
    v.insert("#anon_at-rules_0".to_string(), ContextId { index: 8842 });
    v.insert("#anon_at-supports-parens_0".to_string(), ContextId { index: 8871 });
    v.insert("__main".to_string(), ContextId { index: 8883 });
    v.insert("#anon_at-rules_2".to_string(), ContextId { index: 8854 });
    v.insert("#anon_at-rules_21".to_string(), ContextId { index: 8856 });
    v.insert("#anon_at-rules_23".to_string(), ContextId { index: 8858 });
    v.insert("#anon_at-rules_24".to_string(), ContextId { index: 8859 });
    v.insert("#anon_at-rules_17".to_string(), ContextId { index: 8851 });
    v.insert("#anon_at-rules_28".to_string(), ContextId { index: 8863 });
    v.insert("#anon_at-rules_4".to_string(), ContextId { index: 8865 });
    v.insert("properties".to_string(), ContextId { index: 8891 });
    v.insert("#anon_properties_2".to_string(), ContextId { index: 8876 });
    v.insert("__start".to_string(), ContextId { index: 8884 });
    v.insert("#anon_properties_5".to_string(), ContextId { index: 8879 });
    v.insert("#anon_properties_6".to_string(), ContextId { index: 8880 });
    v.insert("#anon_at-rules_16".to_string(), ContextId { index: 8850 });
    v.insert("#anon_at-rules_13".to_string(), ContextId { index: 8847 });
    v.insert("#anon_at-rules_27".to_string(), ContextId { index: 8862 });
    v.insert("#anon_at-rules_5".to_string(), ContextId { index: 8866 });
    v.insert("#anon_at-rules_9".to_string(), ContextId { index: 8870 });
    v.insert("#anon_properties_4".to_string(), ContextId { index: 8878 });
    v.insert("#anon_properties_7".to_string(), ContextId { index: 8881 });
    v.insert("at-supports-parens".to_string(), ContextId { index: 8887 });
    v.insert("#anon_properties_3".to_string(), ContextId { index: 8877 });
    v.insert("property-value-wrapper".to_string(), ContextId { index: 8892 });
    v.insert("#anon_at-rules_12".to_string(), ContextId { index: 8846 });
    v.insert("#anon_at-rules_25".to_string(), ContextId { index: 8860 });
    v.insert("#anon_at-rules_15".to_string(), ContextId { index: 8849 });
    v.insert("#anon_properties_1".to_string(), ContextId { index: 8875 });
    v.insert("#anon_at-rules_1".to_string(), ContextId { index: 8843 });
    v.insert("#anon_media-query_0".to_string(), ContextId { index: 8872 });
    v.insert("at-rules".to_string(), ContextId { index: 8886 });
    v.insert("#anon_at-rules_14".to_string(), ContextId { index: 8848 });
    v.insert("#anon_at-rules_18".to_string(), ContextId { index: 8852 });
    v.insert("#anon_media-query_1".to_string(), ContextId { index: 8873 });
    v.insert("#anon_at-rules_8".to_string(), ContextId { index: 8869 });
    v
  }
} }