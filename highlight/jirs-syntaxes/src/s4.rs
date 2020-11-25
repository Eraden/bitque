
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "AppleScript".to_string(),
  file_extensions: vec!["applescript".to_string(),"script editor".to_string()],
  scope: Scope { a: 844459289870336, b: 0 },
  first_line_match: Some("^#!.*(osascript)".to_string()),
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("attributes.considering-ignoring".to_string(), ContextId { index: 144 });
    v.insert("#anon_blocks.tell_1".to_string(), ContextId { index: 125 });
    v.insert("built-in".to_string(), ContextId { index: 150 });
    v.insert("#anon_blocks.other_1".to_string(), ContextId { index: 102 });
    v.insert("standardadditions".to_string(), ContextId { index: 163 });
    v.insert("#anon_blocks.tell_2".to_string(), ContextId { index: 126 });
    v.insert("#anon_blocks.other_5".to_string(), ContextId { index: 108 });
    v.insert("#anon_blocks_3".to_string(), ContextId { index: 135 });
    v.insert("built-in.constant".to_string(), ContextId { index: 151 });
    v.insert("#anon_blocks.statement_0".to_string(), ContextId { index: 118 });
    v.insert("#anon_blocks.tell_6".to_string(), ContextId { index: 130 });
    v.insert("#anon_blocks.statement_3".to_string(), ContextId { index: 121 });
    v.insert("#anon_blocks.repeat_2".to_string(), ContextId { index: 115 });
    v.insert("#anon_blocks.other_11".to_string(), ContextId { index: 104 });
    v.insert("#anon_blocks.statement_4".to_string(), ContextId { index: 122 });
    v.insert("#anon_blocks.tell_4".to_string(), ContextId { index: 128 });
    v.insert("built-in.punctuation".to_string(), ContextId { index: 153 });
    v.insert("built-in.keyword".to_string(), ContextId { index: 152 });
    v.insert("#anon_blocks.tell_0".to_string(), ContextId { index: 124 });
    v.insert("standard-suite".to_string(), ContextId { index: 162 });
    v.insert("#anon_blocks.tell_7".to_string(), ContextId { index: 131 });
    v.insert("#anon_blocks_1".to_string(), ContextId { index: 133 });
    v.insert("main".to_string(), ContextId { index: 161 });
    v.insert("__start".to_string(), ContextId { index: 143 });
    v.insert("#anon_data-structures_3".to_string(), ContextId { index: 141 });
    v.insert("#anon_blocks.other_8".to_string(), ContextId { index: 111 });
    v.insert("blocks.statement".to_string(), ContextId { index: 148 });
    v.insert("#anon_blocks.tell_5".to_string(), ContextId { index: 129 });
    v.insert("#anon_blocks.tell_3".to_string(), ContextId { index: 127 });
    v.insert("inline".to_string(), ContextId { index: 159 });
    v.insert("#anon_blocks.other_0".to_string(), ContextId { index: 101 });
    v.insert("#anon_blocks.repeat_0".to_string(), ContextId { index: 113 });
    v.insert("system-events".to_string(), ContextId { index: 164 });
    v.insert("#anon_blocks.other_7".to_string(), ContextId { index: 110 });
    v.insert("#anon_blocks.repeat_4".to_string(), ContextId { index: 117 });
    v.insert("#anon_blocks.statement_1".to_string(), ContextId { index: 119 });
    v.insert("blocks".to_string(), ContextId { index: 145 });
    v.insert("#anon_data-structures_2".to_string(), ContextId { index: 140 });
    v.insert("#anon_comments.nested_0".to_string(), ContextId { index: 136 });
    v.insert("blocks.other".to_string(), ContextId { index: 146 });
    v.insert("finder".to_string(), ContextId { index: 158 });
    v.insert("#anon_blocks.repeat_1".to_string(), ContextId { index: 114 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 137 });
    v.insert("blocks.tell".to_string(), ContextId { index: 149 });
    v.insert("data-structures".to_string(), ContextId { index: 157 });
    v.insert("blocks.repeat".to_string(), ContextId { index: 147 });
    v.insert("itunes".to_string(), ContextId { index: 160 });
    v.insert("#anon_blocks.other_4".to_string(), ContextId { index: 107 });
    v.insert("#anon_blocks.statement_5".to_string(), ContextId { index: 123 });
    v.insert("#anon_blocks.statement_2".to_string(), ContextId { index: 120 });
    v.insert("__main".to_string(), ContextId { index: 142 });
    v.insert("comments.nested".to_string(), ContextId { index: 156 });
    v.insert("#anon_blocks.repeat_3".to_string(), ContextId { index: 116 });
    v.insert("#anon_blocks.other_9".to_string(), ContextId { index: 112 });
    v.insert("#anon_blocks_2".to_string(), ContextId { index: 134 });
    v.insert("textmate".to_string(), ContextId { index: 165 });
    v.insert("#anon_blocks.other_10".to_string(), ContextId { index: 103 });
    v.insert("#anon_blocks.other_3".to_string(), ContextId { index: 106 });
    v.insert("#anon_blocks.other_6".to_string(), ContextId { index: 109 });
    v.insert("#anon_blocks_0".to_string(), ContextId { index: 132 });
    v.insert("#anon_data-structures_0".to_string(), ContextId { index: 138 });
    v.insert("#anon_data-structures_1".to_string(), ContextId { index: 139 });
    v.insert("#anon_blocks.other_2".to_string(), ContextId { index: 105 });
    v.insert("comments".to_string(), ContextId { index: 155 });
    v.insert("built-in.support".to_string(), ContextId { index: 154 });
    v
  }
} }