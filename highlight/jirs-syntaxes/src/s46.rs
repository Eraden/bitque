
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Makefile".to_string(),
  file_extensions: vec!["make".to_string(),"GNUmakefile".to_string(),"makefile".to_string(),"Makefile".to_string(),"makefile.am".to_string(),"Makefile.am".to_string(),"makefile.in".to_string(),"Makefile.in".to_string(),"OCamlMakefile".to_string(),"mak".to_string(),"mk".to_string()],
  scope: Scope { a: 844631088562176, b: 0 },
  first_line_match: Some("(?xi:\n  ^\\#! .* \\bmake\\b |                     # shebang\n  ^\\# \\s* -\\*- [^*]* makefile [^*]* -\\*- # editorconfig\n)".to_string()),
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("nps_unnested".to_string(), "[^()]*".to_string());
    v.insert("nps".to_string(), "[^()]*(?=[()])".to_string());
    v.insert("open".to_string(), "(?:{{nps}}\\(".to_string());
    v.insert("var_lookahead_base".to_string(), "{{just_eat}}({{varassign}}|{{shellassign}}){{just_eat}}".to_string());
    v.insert("ruleassign".to_string(), ":(?!=)".to_string());
    v.insert("startdirective".to_string(), "ifn?(def|eq)".to_string());
    v.insert("just_eat".to_string(), "(?x)(?:              # ignore whitespace in this regex\n  {{open}}        # start level 1                      __\n    {{open}}      # start level 2      ___/ _____ \\__/ /\n      {{open}}    # start level 3     is like snek... (by Valerie Haecky)\n        {{open}}  # start level 4\n          {{nps}} #       level 4\n        {{close}} #   end level 4\n      {{close}}   #   end level 3\n      {{open}}    # start level 3\n        {{open}}  # start level 4\n          {{nps}} #       level 4\n        {{close}} #   end level 4\n      {{close}}   #   end level 3\n      {{nps}}\n    {{close}}     #   end level 2\n    {{open}}      # start level 2\n      {{open}}    # start level 3\n        {{open}}  # start level 4\n          {{nps}} #       level 4\n        {{close}} #   end level 4\n      {{nps}}\n      {{close}}   #   end level 3\n      {{open}}    # start level 3\n        {{open}}  # start level 4\n          {{nps}} #       level 4\n        {{close}} #   end level 4\n      {{nps}}\n      {{close}}   #   end level 3\n      {{open}}    # start level 3\n        {{open}}  # start level 4\n          {{nps}} #       level 4\n        {{close}} #   end level 4\n      {{nps}}\n      {{close}}   #   end level 3\n    {{nps}}\n    {{close}}     #   end level 2\n  {{nps}}\n  {{close}}       #   end level 1\n  |{{nps_unnested}})\n".to_string());
    v.insert("first_assign_then_colon".to_string(), "(?x)\n  {{just_eat}}\n    {{varassign}}\n  {{just_eat}}\n    {{ruleassign}}\n  {{just_eat}}\n".to_string());
    v.insert("include".to_string(), "[s-]?include".to_string());
    v.insert("function_call_token_begin".to_string(), "\\$\\$?\\(".to_string());
    v.insert("rule_lookahead".to_string(), "{{just_eat}}{{ruleassign}}{{just_eat}}".to_string());
    v.insert("var_lookahead".to_string(), "(?!{{rule_lookahead}}){{var_lookahead_base}}".to_string());
    v.insert("varassign".to_string(), "(\\?|\\+|::?)?=".to_string());
    v.insert("close".to_string(), "\\){{nps_unnested}})?".to_string());
    v.insert("shellassign".to_string(), "!=".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_value-to-be-defined_0".to_string(), ContextId { index: 3158 });
    v.insert("#anon_function-invocations_3".to_string(), ContextId { index: 3141 });
    v.insert("#anon_quoted-string_0".to_string(), ContextId { index: 3151 });
    v.insert("#anon_control-flow_1".to_string(), ContextId { index: 3127 });
    v.insert("#anon_value-maybe-shellscript_0".to_string(), ContextId { index: 3156 });
    v.insert("#anon_inside-control-flow_0".to_string(), ContextId { index: 3145 });
    v.insert("inside-function-call".to_string(), ContextId { index: 3183 });
    v.insert("#anon_function-invocations_6".to_string(), ContextId { index: 3144 });
    v.insert("continuation-or-pop-on-line-end".to_string(), ContextId { index: 3172 });
    v.insert("expect-rule".to_string(), ContextId { index: 3176 });
    v.insert("shell".to_string(), ContextId { index: 3193 });
    v.insert("#anon_shell_0".to_string(), ContextId { index: 3154 });
    v.insert("#anon_line-continuation_0".to_string(), ContextId { index: 3150 });
    v.insert("#anon_expect-rule_5".to_string(), ContextId { index: 3136 });
    v.insert("#anon_expect-rule_3".to_string(), ContextId { index: 3134 });
    v.insert("recipe-common".to_string(), ContextId { index: 3188 });
    v.insert("__main".to_string(), ContextId { index: 3168 });
    v.insert("variable-sub-common".to_string(), ContextId { index: 3198 });
    v.insert("#anon_control-flow_3".to_string(), ContextId { index: 3129 });
    v.insert("eat-whitespace-then-pop".to_string(), ContextId { index: 3174 });
    v.insert("value-maybe-shellscript".to_string(), ContextId { index: 3195 });
    v.insert("highlight-percentage-sign".to_string(), ContextId { index: 3178 });
    v.insert("#anon_function-invocations_5".to_string(), ContextId { index: 3143 });
    v.insert("recipe-with-tabs".to_string(), ContextId { index: 3192 });
    v.insert("comments-pop-on-line-end".to_string(), ContextId { index: 3171 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 3123 });
    v.insert("#anon_variable-definitions_4".to_string(), ContextId { index: 3164 });
    v.insert("value-to-be-defined".to_string(), ContextId { index: 3196 });
    v.insert("#anon_function-invocations_1".to_string(), ContextId { index: 3139 });
    v.insert("inside-define-directive-value".to_string(), ContextId { index: 3182 });
    v.insert("recipe-inline".to_string(), ContextId { index: 3189 });
    v.insert("#anon_control-flow_4".to_string(), ContextId { index: 3130 });
    v.insert("highlight-wildcard-sign".to_string(), ContextId { index: 3179 });
    v.insert("recipe-with-spaces".to_string(), ContextId { index: 3191 });
    v.insert("#anon_expect-rule_1".to_string(), ContextId { index: 3132 });
    v.insert("#anon_value-to-be-defined_1".to_string(), ContextId { index: 3159 });
    v.insert("#anon_control-flow_0".to_string(), ContextId { index: 3126 });
    v.insert("#anon_shell_1".to_string(), ContextId { index: 3155 });
    v.insert("escape-literals".to_string(), ContextId { index: 3175 });
    v.insert("variable-substitutions".to_string(), ContextId { index: 3199 });
    v.insert("__start".to_string(), ContextId { index: 3169 });
    v.insert("inside-control-flow".to_string(), ContextId { index: 3180 });
    v.insert("comments".to_string(), ContextId { index: 3170 });
    v.insert("main".to_string(), ContextId { index: 3185 });
    v.insert("#anon_expect-rule_0".to_string(), ContextId { index: 3131 });
    v.insert("#anon_inside-define-directive-context_0".to_string(), ContextId { index: 3148 });
    v.insert("#anon_variable-substitutions_0".to_string(), ContextId { index: 3166 });
    v.insert("textual-parenthesis-balancer".to_string(), ContextId { index: 3194 });
    v.insert("#anon_variable-definitions_2".to_string(), ContextId { index: 3162 });
    v.insert("#anon_function-invocations_2".to_string(), ContextId { index: 3140 });
    v.insert("variable-definitions".to_string(), ContextId { index: 3197 });
    v.insert("#anon_expect-rule_4".to_string(), ContextId { index: 3135 });
    v.insert("#anon_variable-definitions_1".to_string(), ContextId { index: 3161 });
    v.insert("quoted-string".to_string(), ContextId { index: 3187 });
    v.insert("#anon_inside-define-directive-value_0".to_string(), ContextId { index: 3149 });
    v.insert("#anon_recipe-junction-between-spaces-or-tabs_0".to_string(), ContextId { index: 3153 });
    v.insert("#anon_control-flow_2".to_string(), ContextId { index: 3128 });
    v.insert("#anon_variable-definitions_5".to_string(), ContextId { index: 3165 });
    v.insert("#anon_expect-rule_6".to_string(), ContextId { index: 3137 });
    v.insert("line-continuation".to_string(), ContextId { index: 3184 });
    v.insert("#anon_comments_1".to_string(), ContextId { index: 3124 });
    v.insert("recipe-junction-between-spaces-or-tabs".to_string(), ContextId { index: 3190 });
    v.insert("#anon_function-invocations_4".to_string(), ContextId { index: 3142 });
    v.insert("#anon_quoted-string_1".to_string(), ContextId { index: 3152 });
    v.insert("inside-define-directive-context".to_string(), ContextId { index: 3181 });
    v.insert("#anon_inside-control-flow_1".to_string(), ContextId { index: 3146 });
    v.insert("pop-on-line-end".to_string(), ContextId { index: 3186 });
    v.insert("#anon_function-invocations_0".to_string(), ContextId { index: 3138 });
    v.insert("#anon_variable-definitions_3".to_string(), ContextId { index: 3163 });
    v.insert("#anon_inside-control-flow_2".to_string(), ContextId { index: 3147 });
    v.insert("#anon_variable-substitutions_1".to_string(), ContextId { index: 3167 });
    v.insert("#anon_comments_2".to_string(), ContextId { index: 3125 });
    v.insert("#anon_value-maybe-shellscript_1".to_string(), ContextId { index: 3157 });
    v.insert("control-flow".to_string(), ContextId { index: 3173 });
    v.insert("#anon_expect-rule_2".to_string(), ContextId { index: 3133 });
    v.insert("function-invocations".to_string(), ContextId { index: 3177 });
    v.insert("#anon_variable-definitions_0".to_string(), ContextId { index: 3160 });
    v
  }
} }