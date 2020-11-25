use jirs_syntect::*;
use jirs_syntect::highlighting::*;
pub fn load() -> Theme {
Theme::new(  Some(
    "Nord",
),
  Some(
    "Arctic Ice Studio",
),
  ThemeSettings {
    foreground: Some(
        Color {
            r: 216,
            g: 222,
            b: 233,
            a: 255,
        },
    ),
    background: Some(
        Color {
            r: 46,
            g: 52,
            b: 64,
            a: 255,
        },
    ),
    caret: Some(
        Color {
            r: 216,
            g: 222,
            b: 233,
            a: 255,
        },
    ),
    line_highlight: Some(
        Color {
            r: 67,
            g: 76,
            b: 94,
            a: 82,
        },
    ),
    misspelling: Some(
        Color {
            r: 191,
            g: 97,
            b: 106,
            a: 255,
        },
    ),
    minimap_border: Some(
        Color {
            r: 67,
            g: 76,
            b: 94,
            a: 82,
        },
    ),
    accent: Some(
        Color {
            r: 136,
            g: 192,
            b: 208,
            a: 255,
        },
    ),
    popup_css: None,
    phantom_css: None,
    bracket_contents_foreground: None,
    bracket_contents_options: None,
    brackets_foreground: Some(
        Color {
            r: 136,
            g: 192,
            b: 208,
            a: 255,
        },
    ),
    brackets_background: None,
    brackets_options: Some(
        UnderlineOption::Underline,
    ),
    tags_foreground: Some(
        Color {
            r: 136,
            g: 192,
            b: 208,
            a: 255,
        },
    ),
    tags_options: Some(
        UnderlineOption::Underline,
    ),
    highlight: Some(
        Color {
            r: 136,
            g: 192,
            b: 208,
            a: 255,
        },
    ),
    find_highlight: Some(
        Color {
            r: 136,
            g: 192,
            b: 208,
            a: 255,
        },
    ),
    find_highlight_foreground: Some(
        Color {
            r: 46,
            g: 52,
            b: 64,
            a: 255,
        },
    ),
    gutter: Some(
        Color {
            r: 46,
            g: 52,
            b: 64,
            a: 255,
        },
    ),
    gutter_foreground: Some(
        Color {
            r: 216,
            g: 222,
            b: 233,
            a: 102,
        },
    ),
    selection: Some(
        Color {
            r: 67,
            g: 76,
            b: 94,
            a: 204,
        },
    ),
    selection_foreground: None,
    selection_background: None,
    selection_border: Some(
        Color {
            r: 67,
            g: 76,
            b: 94,
            a: 204,
        },
    ),
    inactive_selection: Some(
        Color {
            r: 67,
            g: 76,
            b: 94,
            a: 153,
        },
    ),
    inactive_selection_foreground: None,
    guide: Some(
        Color {
            r: 67,
            g: 76,
            b: 94,
            a: 179,
        },
    ),
    active_guide: Some(
        Color {
            r: 136,
            g: 192,
            b: 208,
            a: 204,
        },
    ),
    stack_guide: Some(
        Color {
            r: 136,
            g: 192,
            b: 208,
            a: 102,
        },
    ),
    highlight_foreground: None,
    shadow: Some(
        Color {
            r: 46,
            g: 52,
            b: 64,
            a: 255,
        },
    ),
},
  vec![
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 51509920738050048,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629323038720,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 97,
                g: 110,
                b: 136,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955089162371072,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 180,
                g: 142,
                b: 173,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955110637207552,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955200847314944,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 235,
                g: 203,
                b: 139,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955136434012160,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 235,
                g: 203,
                b: 139,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955136407011328,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 216,
                g: 222,
                b: 233,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130632318976,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130645426176,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130642804736,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130647916544,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130694250496,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130646081536,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130632450048,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392186470432768,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130630615040,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 136,
                g: 192,
                b: 208,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130630615404,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 136,
                g: 192,
                b: 208,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130630615473,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 136,
                g: 192,
                b: 208,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130643525632,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130632974336,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130642149376,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 94,
                g: 129,
                b: 172,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130630090752,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 136,
                g: 192,
                b: 208,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130632122368,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392186477182976,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 50103314653642752,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 216,
                g: 222,
                b: 233,
                a: 255,
            },
        ), Some(
            Color {
                r: 191,
                g: 97,
                b: 106,
                a: 255,
            },
        ), None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 50104723402915840,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 216,
                g: 222,
                b: 233,
                a: 255,
            },
        ), Some(
            Color {
                r: 235,
                g: 203,
                b: 139,
                a: 255,
            },
        ), None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636636688678912,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636636711616512,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636636717449216,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629323300864,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636787012534272,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636628098744320,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636628111130624,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636628119191552,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636628135903232,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636628114800640,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636628119322624,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114281636568236032,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 136,
                g: 192,
                b: 208,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114280017427103744,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114280017426907136,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 216,
                g: 222,
                b: 233,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114281679517908992,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(None, None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114282585756008448,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(None, None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114281327330590720,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 163,
                g: 190,
                b: 140,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114281335920525312,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 191,
                g: 97,
                b: 106,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114281331625558016,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 235,
                g: 203,
                b: 139,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114282856338948096,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 97,
                g: 110,
                b: 136,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114280120494129152,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114280120459395072,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114279806912167936,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(None, Some(
            Color {
                r: 236,
                g: 239,
                b: 244,
                a: 255,
            },
        ), None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288620721111040,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288689440587776,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 236,
                g: 239,
                b: 244,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288620736970752,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288788224835584,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629324152832,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 48414576462528512,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 48414439023575040,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55451420831973376,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55451420828565504,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55451420907143168,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55451420830269440,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55451949096501248,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 163,
                g: 190,
                b: 140,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629323956406,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629323956395,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 163,
                g: 190,
                b: 140,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55450759390560256,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 235,
                g: 203,
                b: 139,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925409704378368,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925255085555712,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 136,
                g: 192,
                b: 208,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61927771936391168,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925375344640000,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925366754705408,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496453775360,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281818574094336,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 236,
                g: 239,
                b: 244,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 49259087292006400,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 49259087310290944,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 216,
                g: 222,
                b: 233,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629322514432,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 49259087305965568,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 49259061522202624,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 49258876838608896,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 216,
                g: 222,
                b: 233,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 49259087306883072,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 216,
                g: 222,
                b: 233,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 49258881133576192,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 136,
                g: 192,
                b: 208,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955089199794731,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 216,
                g: 222,
                b: 233,
                a: 102,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955089199792867,
                    b: 732679364377837568,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 136,
                g: 192,
                b: 208,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130656373291,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 292654776582144,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 216,
                g: 222,
                b: 233,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844480764706816,
                    b: 0,
                },
                Scope {
                    a: 52636636717449670,
                    b: 3659174697238528,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844480764706816,
                    b: 0,
                },
                Scope {
                    a: 46444466374770688,
                    b: 0,
                },
                Scope {
                    a: 52636636717449229,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844476469739520,
                    b: 0,
                },
                Scope {
                    a: 52636636717449670,
                    b: 3377699720527872,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844476469739520,
                    b: 0,
                },
                Scope {
                    a: 46444466374705152,
                    b: 0,
                },
                Scope {
                    a: 52636636717449228,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 94,
                g: 129,
                b: 172,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844480764706816,
                    b: 0,
                },
                Scope {
                    a: 52636628119322637,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844476469739520,
                    b: 0,
                },
                Scope {
                    a: 52636628119322636,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 136,
                g: 192,
                b: 208,
                a: 255,
            },
        ), None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844485059674112,
                    b: 0,
                },
                Scope {
                    a: 61925375377670158,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845030520520704,
                    b: 0,
                },
                Scope {
                    a: 61925375377670285,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844485059674112,
                    b: 0,
                },
                Scope {
                    a: 61925375377735694,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 216,
                g: 222,
                b: 233,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844485059674112,
                    b: 0,
                },
                Scope {
                    a: 59955136441680405,
                    b: 3940649673949184,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845030520520704,
                    b: 0,
                },
                Scope {
                    a: 59955136441942157,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 180,
                g: 142,
                b: 173,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844485059674112,
                    b: 0,
                },
                Scope {
                    a: 61925409739047444,
                    b: 3940649673949184,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844485059674112,
                    b: 0,
                },
                Scope {
                    a: 61925409737015310,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845030520520704,
                    b: 0,
                },
                Scope {
                    a: 61925409737015437,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844485059674112,
                    b: 0,
                },
                Scope {
                    a: 59392186486751246,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844485059674112,
                    b: 0,
                },
                Scope {
                    a: 52636636720398829,
                    b: 3940649673949184,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844485059674112,
                    b: 0,
                },
                Scope {
                    a: 47288629323300878,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845030520520704,
                    b: 0,
                },
                Scope {
                    a: 52636636720398477,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 208,
                g: 135,
                b: 112,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844485059674112,
                    b: 0,
                },
                Scope {
                    a: 61925409739047442,
                    b: 3940649673949184,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844485059674112,
                    b: 0,
                },
                Scope {
                    a: 59392186477183489,
                    b: 3940649673949184,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845030520520704,
                    b: 0,
                },
                Scope {
                    a: 59392186477183489,
                    b: 39687971716202496,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(None, None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844506534510592,
                    b: 0,
                },
                Scope {
                    a: 46443452779528767,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844506534510592,
                    b: 0,
                },
                Scope {
                    a: 46443452799386177,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844506534510592,
                    b: 0,
                },
                Scope {
                    a: 47288629348859904,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844506534510592,
                    b: 0,
                },
                Scope {
                    a: 47288629323628544,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844558074118144,
                    b: 0,
                },
                Scope {
                    a: 61925255090602015,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(None, None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496453775360,
                    b: 0,
                },
                Scope {
                    a: 59955200845348869,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 235,
                g: 203,
                b: 139,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 49259830323970048,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629337129000,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 208,
                g: 135,
                b: 112,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288788226932776,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 236,
                g: 239,
                b: 244,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925461290385448,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844596728823808,
                    b: 0,
                },
                Scope {
                    a: 51510878542233641,
                    b: 0,
                },
                Scope {
                    a: 47288629324873733,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844596728823808,
                    b: 0,
                },
                Scope {
                    a: 59392130632974336,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 216,
                g: 222,
                b: 233,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844596728823808,
                    b: 0,
                },
                Scope {
                    a: 52636787039010816,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844596728823808,
                    b: 0,
                },
                Scope {
                    a: 51510878542233641,
                    b: 0,
                },
                Scope {
                    a: 47288629323300905,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844596728823808,
                    b: 0,
                },
                Scope {
                    a: 49258876852043816,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844596728823808,
                    b: 0,
                },
                Scope {
                    a: 59392186470432808,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844596728823808,
                    b: 0,
                },
                Scope {
                    a: 52638212988796968,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844588138889216,
                    b: 0,
                },
                Scope {
                    a: 46446510938914854,
                    b: 0,
                },
                Scope {
                    a: 46446510938915111,
                    b: 10696049115004928,
                },
                Scope {
                    a: 55451420828565542,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844588311117824,
                    b: 0,
                },
                Scope {
                    a: 46446510938914854,
                    b: 0,
                },
                Scope {
                    a: 46446510938915111,
                    b: 10696049115004928,
                },
                Scope {
                    a: 55451420828565542,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844588138889216,
                    b: 0,
                },
                Scope {
                    a: 46446510938914854,
                    b: 0,
                },
                Scope {
                    a: 47288629323956406,
                    b: 10696049115004928,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844588138889216,
                    b: 0,
                },
                Scope {
                    a: 46446510938914854,
                    b: 0,
                },
                Scope {
                    a: 47288629323956395,
                    b: 10696049115004928,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844588311117824,
                    b: 0,
                },
                Scope {
                    a: 46446510938914854,
                    b: 0,
                },
                Scope {
                    a: 47288629323956406,
                    b: 10696049115004928,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844588311117824,
                    b: 0,
                },
                Scope {
                    a: 46446510938914854,
                    b: 0,
                },
                Scope {
                    a: 47288629323956395,
                    b: 10696049115004928,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 236,
                g: 239,
                b: 244,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 47288629330576302,
                    b: 51228656214736896,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 47288629330576302,
                    b: 48132431470919680,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 114280120506515505,
                    b: 0,
                },
                Scope {
                    a: 59955136470515761,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 47288629372518449,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 114280017426908073,
                    b: 13792273858822144,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 114282856342159360,
                    b: 0,
                },
                Scope {
                    a: 47288629374287921,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 46443487170724718,
                    b: 13792273858822144,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 46445673309078382,
                    b: 13792273858822144,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 46443487132779374,
                    b: 13792273858822144,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 59955136418414619,
                    b: 13792273858822144,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 59392130630418459,
                    b: 13792273858822144,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 136,
                g: 192,
                b: 208,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 47288629312815286,
                    b: 13792273858822144,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 47288629312815275,
                    b: 13792273858822144,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 47288629346173110,
                    b: 13792273858822144,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 47288629346173099,
                    b: 13792273858822144,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 47288629325004982,
                    b: 13792273858822144,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 47288629325004971,
                    b: 13792273858822144,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 47288620783763505,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 47288521999712305,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 46446613895315505,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 136,
                g: 192,
                b: 208,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281496454758400,
                    b: 0,
                },
                Scope {
                    a: 47288629374025777,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 16607272734031872,
                    b: 0,
                },
                Scope {
                    a: 281496453775360,
                    b: 0,
                },
                Scope {
                    a: 47288521949642934,
                    b: 16325548649218048,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 16607272734031872,
                    b: 0,
                },
                Scope {
                    a: 281496453775360,
                    b: 0,
                },
                Scope {
                    a: 47288521949642923,
                    b: 16325548649218048,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 94,
                g: 129,
                b: 172,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 16607272734031872,
                    b: 0,
                },
                Scope {
                    a: 281496453775360,
                    b: 0,
                },
                Scope {
                    a: 51510878542234726,
                    b: 16325548649218048,
                },
                Scope {
                    a: 52636787086327866,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 16607272734031872,
                    b: 0,
                },
                Scope {
                    a: 281496453775360,
                    b: 0,
                },
                Scope {
                    a: 61925461268496442,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844691218104320,
                    b: 0,
                },
                Scope {
                    a: 46445080558305280,
                    b: 0,
                },
                Scope {
                    a: 46448842949656576,
                    b: 0,
                },
                Scope {
                    a: 61925255090602046,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844691218104320,
                    b: 0,
                },
                Scope {
                    a: 47288629337129022,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 208,
                g: 135,
                b: 112,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925255090602046,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(None, None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844691218104320,
                    b: 0,
                },
                Scope {
                    a: 61925375362138174,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844691218104320,
                    b: 0,
                },
                Scope {
                    a: 61925375348703232,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845030520520704,
                    b: 0,
                },
                Scope {
                    a: 49258876847849472,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 216,
                g: 222,
                b: 233,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844708397973504,
                    b: 0,
                },
                Scope {
                    a: 61925255090602050,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(None, None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844742757711872,
                    b: 0,
                },
                Scope {
                    a: 61925255090602058,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(None, None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281818574094336,
                    b: 0,
                },
                Scope {
                    a: 59392130632122742,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281818574094336,
                    b: 0,
                },
                Scope {
                    a: 46444230199739128,
                    b: 22517998136852480,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281818574094336,
                    b: 0,
                },
                Scope {
                    a: 46444230167429120,
                    b: 0,
                },
                Scope {
                    a: 59392130632122368,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281818574094336,
                    b: 0,
                },
                Scope {
                    a: 46444230167429200,
                    b: 0,
                },
                Scope {
                    a: 47288629324153014,
                    b: 22517998136852480,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281818574094336,
                    b: 0,
                },
                Scope {
                    a: 46444230167429200,
                    b: 0,
                },
                Scope {
                    a: 47288629324153003,
                    b: 22517998136852480,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281818574094336,
                    b: 0,
                },
                Scope {
                    a: 46444230199739128,
                    b: 22517998136852480,
                },
                Scope {
                    a: 47288629324153014,
                    b: 22517998136852480,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281818574094336,
                    b: 0,
                },
                Scope {
                    a: 46444230199739128,
                    b: 22517998136852480,
                },
                Scope {
                    a: 47288629324153003,
                    b: 22517998136852480,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 94,
                g: 129,
                b: 172,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281818574094336,
                    b: 0,
                },
                Scope {
                    a: 55451949145587792,
                    b: 0,
                },
                Scope {
                    a: 47288629323956406,
                    b: 22517998136852480,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281818574094336,
                    b: 0,
                },
                Scope {
                    a: 55451949145587792,
                    b: 0,
                },
                Scope {
                    a: 47288629323956395,
                    b: 22517998136852480,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 208,
                g: 135,
                b: 112,
                a: 255,
            },
        ), None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844772822482944,
                    b: 0,
                },
                Scope {
                    a: 59392130632122368,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 737757188273471488,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 235,
                g: 203,
                b: 139,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 737757188265410560,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 191,
                g: 97,
                b: 106,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114280326773407744,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 97,
                g: 110,
                b: 136,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114289999039758336,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 136,
                g: 192,
                b: 208,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114281327500787712,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 163,
                g: 190,
                b: 140,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114281331795755008,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 235,
                g: 203,
                b: 139,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114281336090722304,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 191,
                g: 97,
                b: 106,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844609613725696,
                    b: 0,
                },
                Scope {
                    a: 56299706924072960,
                    b: 0,
                },
                Scope {
                    a: 59392130632122411,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844609613725696,
                    b: 0,
                },
                Scope {
                    a: 56299706924072960,
                    b: 0,
                },
                Scope {
                    a: 47288629324152875,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 208,
                g: 135,
                b: 112,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844609613725696,
                    b: 0,
                },
                Scope {
                    a: 55451420818341888,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844609613725696,
                    b: 0,
                },
                Scope {
                    a: 55451420969992192,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844609613725696,
                    b: 0,
                },
                Scope {
                    a: 46444243038437376,
                    b: 0,
                },
                Scope {
                    a: 46444243050889259,
                    b: 0,
                },
                Scope {
                    a: 55452159552716800,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844609613725696,
                    b: 0,
                },
                Scope {
                    a: 55452159552716800,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844609613725696,
                    b: 0,
                },
                Scope {
                    a: 55452468790362112,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 163,
                g: 190,
                b: 140,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844609613725696,
                    b: 0,
                },
                Scope {
                    a: 49259087305965611,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 216,
                g: 222,
                b: 233,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844609613725696,
                    b: 0,
                },
                Scope {
                    a: 59955136461406503,
                    b: 12103423998558208,
                },
                Scope {
                    a: 55451949119635499,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844609613725696,
                    b: 0,
                },
                Scope {
                    a: 49259087310290987,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 216,
                g: 222,
                b: 233,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844609613725696,
                    b: 0,
                },
                Scope {
                    a: 46444243038437376,
                    b: 0,
                },
                Scope {
                    a: 46444243050889259,
                    b: 0,
                },
                Scope {
                    a: 49259087307276331,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 136,
                g: 192,
                b: 208,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844609613725696,
                    b: 0,
                },
                Scope {
                    a: 49259087307276771,
                    b: 12103423998558208,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(None, None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844609613725696,
                    b: 0,
                },
                Scope {
                    a: 49259087305310251,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844609613725696,
                    b: 0,
                },
                Scope {
                    a: 55452159552716800,
                    b: 0,
                },
                Scope {
                    a: 52636787035406518,
                    b: 12103423998558208,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844609613725696,
                    b: 0,
                },
                Scope {
                    a: 55452159552716800,
                    b: 0,
                },
                Scope {
                    a: 52636787035406507,
                    b: 12103423998558208,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844609613725696,
                    b: 0,
                },
                Scope {
                    a: 47288629477441718,
                    b: 12103423998558208,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844609613725696,
                    b: 0,
                },
                Scope {
                    a: 47288629477441707,
                    b: 12103423998558208,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 94,
                g: 129,
                b: 172,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844609613725696,
                    b: 0,
                },
                Scope {
                    a: 52636628152877099,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844609613725696,
                    b: 0,
                },
                Scope {
                    a: 46445969615290368,
                    b: 0,
                },
                Scope {
                    a: 49259087310290987,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845069175226368,
                    b: 0,
                },
                Scope {
                    a: 46444243045449728,
                    b: 0,
                },
                Scope {
                    a: 59392186470432918,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845069175226368,
                    b: 0,
                },
                Scope {
                    a: 59392130632450251,
                    b: 42221246506598400,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845069175226368,
                    b: 0,
                },
                Scope {
                    a: 59392130632450411,
                    b: 42221246506598400,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845069175226368,
                    b: 0,
                },
                Scope {
                    a: 59392130632450198,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845069175226368,
                    b: 0,
                },
                Scope {
                    a: 61925375393464470,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 129,
                g: 161,
                b: 193,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845069175226368,
                    b: 0,
                },
                Scope {
                    a: 46444243045449728,
                    b: 0,
                },
                Scope {
                    a: 46444084158333078,
                    b: 0,
                },
                Scope {
                    a: 46444453498847232,
                    b: 0,
                },
                Scope {
                    a: 46448082746212352,
                    b: 0,
                },
                Scope {
                    a: 49259087310291094,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845069175226368,
                    b: 0,
                },
                Scope {
                    a: 46444243045449728,
                    b: 0,
                },
                Scope {
                    a: 46444084158333078,
                    b: 0,
                },
                Scope {
                    a: 46444453498847232,
                    b: 0,
                },
                Scope {
                    a: 46448082746212352,
                    b: 0,
                },
                Scope {
                    a: 47292507676344320,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 208,
                g: 135,
                b: 112,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845069175226368,
                    b: 0,
                },
                Scope {
                    a: 47288629477441718,
                    b: 42221246506598400,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845069175226368,
                    b: 0,
                },
                Scope {
                    a: 47288629477441707,
                    b: 42221246506598400,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 94,
                g: 129,
                b: 172,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845069175226368,
                    b: 0,
                },
                Scope {
                    a: 55452468797374464,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 163,
                g: 190,
                b: 140,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845069175226368,
                    b: 0,
                },
                Scope {
                    a: 61925255142441110,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(None, None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845069175226368,
                    b: 0,
                },
                Scope {
                    a: 49259087305965801,
                    b: 42221246506598400,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845069175226368,
                    b: 0,
                },
                Scope {
                    a: 49259087457681558,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845069175226368,
                    b: 0,
                },
                Scope {
                    a: 49259087305965718,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 216,
                g: 222,
                b: 233,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 845069175226368,
                    b: 0,
                },
                Scope {
                    a: 51510878542233750,
                    b: 0,
                },
                Scope {
                    a: 48414576475834846,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 188,
                b: 187,
                a: 255,
            },
        ), None, None),
    },
]
)}
