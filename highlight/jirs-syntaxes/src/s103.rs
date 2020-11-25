
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Dockerfile".to_string(),
  file_extensions: vec!["Dockerfile".to_string(),"dockerfile".to_string()],
  scope: Scope { a: 844841541959680, b: 0 },
  first_line_match: Some("^\\s*(?i:(from(?!\\s+\\S+\\s+import)|arg))\\s+".to_string()),
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("onbuild_commands_directive".to_string(), "{{onbuild_directive}}(?i:add|arg|env|expose|healthcheck|label|run|shell|stopsignal|user|volume|workdir)".to_string());
    v.insert("copy_directive".to_string(), "({{onbuild_directive}}(?i:copy))(?:\\s+--from=(\\S+))?".to_string());
    v.insert("from_directive".to_string(), "(?i:(from))\\s+[^\\s:@]+(?:[:@](\\S+))?(?:\\s+(?i:(as))\\s+(\\S+))?".to_string());
    v.insert("onbuild_directive".to_string(), "(?i:(onbuild)\\s+)?".to_string());
    v.insert("nononbuild_commands_directive".to_string(), "(?i:maintainer)".to_string());
    v.insert("runtime_directive".to_string(), "{{onbuild_directive}}(?i:cmd|entrypoint)".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("args".to_string(), ContextId { index: 6733 });
    v.insert("directives".to_string(), ContextId { index: 6736 });
    v.insert("__start".to_string(), ContextId { index: 6732 });
    v.insert("__main".to_string(), ContextId { index: 6731 });
    v.insert("from".to_string(), ContextId { index: 6739 });
    v.insert("single_quote_string".to_string(), ContextId { index: 6742 });
    v.insert("comments".to_string(), ContextId { index: 6735 });
    v.insert("body".to_string(), ContextId { index: 6734 });
    v.insert("double_quote_string".to_string(), ContextId { index: 6737 });
    v.insert("escaped-char".to_string(), ContextId { index: 6738 });
    v.insert("main".to_string(), ContextId { index: 6741 });
    v.insert("invalid".to_string(), ContextId { index: 6740 });
    v
  }
} }