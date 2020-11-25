
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Crystal".to_string(),
  file_extensions: vec!["cr".to_string()],
  scope: Scope { a: 844828657057792, b: 0 },
  first_line_match: Some("^#!/.*\\bcrystal".to_string()),
  hidden: false,
  variables: std::collections::HashMap::new(),
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_main_5".to_string(), ContextId { index: 6645 });
    v.insert("nest_ltgt_i".to_string(), ContextId { index: 6679 });
    v.insert("nest_parens_r".to_string(), ContextId { index: 6683 });
    v.insert("nest_brackets".to_string(), ContextId { index: 6671 });
    v.insert("#anon_main_12".to_string(), ContextId { index: 6610 });
    v.insert("#anon_main_38".to_string(), ContextId { index: 6638 });
    v.insert("#anon_nest_ltgt_i_0".to_string(), ContextId { index: 6658 });
    v.insert("#anon_main_26".to_string(), ContextId { index: 6625 });
    v.insert("#anon_nest_ltgt_r_0".to_string(), ContextId { index: 6659 });
    v.insert("#anon_nest_parens_r_0".to_string(), ContextId { index: 6662 });
    v.insert("#anon_main_33".to_string(), ContextId { index: 6633 });
    v.insert("#anon_nest_brackets_i_0".to_string(), ContextId { index: 6651 });
    v.insert("main".to_string(), ContextId { index: 6670 });
    v.insert("#anon_main_0".to_string(), ContextId { index: 6606 });
    v.insert("#anon_main_21".to_string(), ContextId { index: 6620 });
    v.insert("nest_brackets_r".to_string(), ContextId { index: 6673 });
    v.insert("#anon_nest_brackets_0".to_string(), ContextId { index: 6650 });
    v.insert("#anon_main_2".to_string(), ContextId { index: 6618 });
    v.insert("#anon_main_19".to_string(), ContextId { index: 6617 });
    v.insert("#anon_main_35".to_string(), ContextId { index: 6635 });
    v.insert("#anon_main_34".to_string(), ContextId { index: 6634 });
    v.insert("#anon_main_13".to_string(), ContextId { index: 6611 });
    v.insert("#anon_heredoc_0".to_string(), ContextId { index: 6604 });
    v.insert("#anon_main_32".to_string(), ContextId { index: 6632 });
    v.insert("#anon_main_17".to_string(), ContextId { index: 6615 });
    v.insert("#anon_main_25".to_string(), ContextId { index: 6624 });
    v.insert("regex_sub".to_string(), ContextId { index: 6684 });
    v.insert("#anon_main_18".to_string(), ContextId { index: 6616 });
    v.insert("#anon_main_30".to_string(), ContextId { index: 6630 });
    v.insert("escaped_char".to_string(), ContextId { index: 6667 });
    v.insert("nest_curly".to_string(), ContextId { index: 6674 });
    v.insert("nest_curly_and_self".to_string(), ContextId { index: 6675 });
    v.insert("#anon_nest_ltgt_0".to_string(), ContextId { index: 6657 });
    v.insert("#anon_main_20".to_string(), ContextId { index: 6619 });
    v.insert("#anon_main_31".to_string(), ContextId { index: 6631 });
    v.insert("#anon_main_42".to_string(), ContextId { index: 6643 });
    v.insert("#anon_main_23".to_string(), ContextId { index: 6622 });
    v.insert("#anon_nest_curly_and_self_0".to_string(), ContextId { index: 6654 });
    v.insert("#anon_main_4".to_string(), ContextId { index: 6640 });
    v.insert("#anon_nest_curly_i_0".to_string(), ContextId { index: 6655 });
    v.insert("#anon_nest_brackets_r_0".to_string(), ContextId { index: 6652 });
    v.insert("nest_curly_i".to_string(), ContextId { index: 6676 });
    v.insert("#anon_main_9".to_string(), ContextId { index: 6649 });
    v.insert("nest_parens".to_string(), ContextId { index: 6681 });
    v.insert("#anon_main_43".to_string(), ContextId { index: 6644 });
    v.insert("#anon_main_3".to_string(), ContextId { index: 6629 });
    v.insert("#anon_main_39".to_string(), ContextId { index: 6639 });
    v.insert("heredoc".to_string(), ContextId { index: 6668 });
    v.insert("nest_ltgt".to_string(), ContextId { index: 6678 });
    v.insert("nest_brackets_i".to_string(), ContextId { index: 6672 });
    v.insert("#anon_main_27".to_string(), ContextId { index: 6626 });
    v.insert("#anon_main_7".to_string(), ContextId { index: 6647 });
    v.insert("nest_curly_r".to_string(), ContextId { index: 6677 });
    v.insert("#anon_main_11".to_string(), ContextId { index: 6609 });
    v.insert("__start".to_string(), ContextId { index: 6666 });
    v.insert("#anon_main_36".to_string(), ContextId { index: 6636 });
    v.insert("#anon_main_1".to_string(), ContextId { index: 6607 });
    v.insert("#anon_main_24".to_string(), ContextId { index: 6623 });
    v.insert("#anon_main_10".to_string(), ContextId { index: 6608 });
    v.insert("#anon_main_29".to_string(), ContextId { index: 6628 });
    v.insert("#anon_main_22".to_string(), ContextId { index: 6621 });
    v.insert("#anon_main_41".to_string(), ContextId { index: 6642 });
    v.insert("#anon_main_8".to_string(), ContextId { index: 6648 });
    v.insert("#anon_main_15".to_string(), ContextId { index: 6613 });
    v.insert("#anon_main_14".to_string(), ContextId { index: 6612 });
    v.insert("#anon_main_6".to_string(), ContextId { index: 6646 });
    v.insert("#anon_nest_parens_i_0".to_string(), ContextId { index: 6661 });
    v.insert("#anon_nest_curly_0".to_string(), ContextId { index: 6653 });
    v.insert("#anon_regex_sub_0".to_string(), ContextId { index: 6663 });
    v.insert("#anon_main_16".to_string(), ContextId { index: 6614 });
    v.insert("#anon_main_40".to_string(), ContextId { index: 6641 });
    v.insert("#anon_regex_sub_1".to_string(), ContextId { index: 6664 });
    v.insert("__main".to_string(), ContextId { index: 6665 });
    v.insert("nest_parens_i".to_string(), ContextId { index: 6682 });
    v.insert("#anon_main_28".to_string(), ContextId { index: 6627 });
    v.insert("#anon_nest_curly_r_0".to_string(), ContextId { index: 6656 });
    v.insert("interpolated_crystal".to_string(), ContextId { index: 6669 });
    v.insert("#anon_nest_parens_0".to_string(), ContextId { index: 6660 });
    v.insert("nest_ltgt_r".to_string(), ContextId { index: 6680 });
    v.insert("#anon_interpolated_crystal_0".to_string(), ContextId { index: 6605 });
    v.insert("#anon_main_37".to_string(), ContextId { index: 6637 });
    v
  }
} }