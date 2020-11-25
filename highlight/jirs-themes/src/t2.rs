use jirs_syntect::*;
use jirs_syntect::highlighting::*;
pub fn load() -> Theme {
Theme::new(  Some(
    "Dracula",
),
  Some(
    "Zeno Rocha",
),
  ThemeSettings {
    foreground: Some(
        Color {
            r: 248,
            g: 248,
            b: 242,
            a: 255,
        },
    ),
    background: Some(
        Color {
            r: 40,
            g: 42,
            b: 54,
            a: 255,
        },
    ),
    caret: Some(
        Color {
            r: 248,
            g: 248,
            b: 240,
            a: 255,
        },
    ),
    line_highlight: Some(
        Color {
            r: 68,
            g: 71,
            b: 90,
            a: 255,
        },
    ),
    misspelling: None,
    minimap_border: None,
    accent: None,
    popup_css: None,
    phantom_css: None,
    bracket_contents_foreground: Some(
        Color {
            r: 248,
            g: 248,
            b: 242,
            a: 165,
        },
    ),
    bracket_contents_options: Some(
        UnderlineOption::Underline,
    ),
    brackets_foreground: Some(
        Color {
            r: 248,
            g: 248,
            b: 242,
            a: 165,
        },
    ),
    brackets_background: None,
    brackets_options: Some(
        UnderlineOption::Underline,
    ),
    tags_foreground: None,
    tags_options: Some(
        UnderlineOption::StippledUnderline,
    ),
    highlight: None,
    find_highlight: Some(
        Color {
            r: 239,
            g: 251,
            b: 123,
            a: 255,
        },
    ),
    find_highlight_foreground: Some(
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        },
    ),
    gutter: None,
    gutter_foreground: None,
    selection: Some(
        Color {
            r: 68,
            g: 71,
            b: 90,
            a: 255,
        },
    ),
    selection_foreground: None,
    selection_background: None,
    selection_border: Some(
        Color {
            r: 34,
            g: 34,
            b: 24,
            a: 255,
        },
    ),
    inactive_selection: None,
    inactive_selection_foreground: None,
    guide: None,
    active_guide: Some(
        Color {
            r: 157,
            g: 85,
            b: 15,
            a: 176,
        },
    ),
    stack_guide: None,
    highlight_foreground: None,
    shadow: None,
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
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 98,
                g: 114,
                b: 164,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55450570411999232,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 241,
                g: 250,
                b: 140,
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
                r: 189,
                g: 147,
                b: 249,
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
                r: 189,
                g: 147,
                b: 249,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955200831520768,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955136407011328,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 189,
                g: 147,
                b: 249,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 49258120924364800,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(None, None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 49259087310291329,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 184,
                b: 108,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955200913375232,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955200847314944,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55450570411999232,
                    b: 0,
                },
                Scope {
                    a: 844424930131968,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55450570411999232,
                    b: 0,
                },
                Scope {
                    a: 844708397973504,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 121,
                b: 198,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844708397973504,
                    b: 0,
                },
                Scope {
                    a: 55450759479820354,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844708397973504,
                    b: 0,
                },
                Scope {
                    a: 55450759479754818,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 85,
                b: 85,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52635820644892672,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 121,
                b: 198,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 48413695994232832,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 121,
                b: 198,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 48414576462528512,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 139,
                g: 233,
                b: 253,
                a: 255,
            },
        ), None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 48414576487038976,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 139,
                g: 233,
                b: 253,
                a: 255,
            },
        ), None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 48414576475832320,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 121,
                b: 198,
                a: 255,
            },
        ), None, FontStyle::from_bits(4)),
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
                r: 139,
                g: 233,
                b: 253,
                a: 255,
            },
        ), None, FontStyle::from_bits(2)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46445252352933888,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 102,
                g: 217,
                b: 239,
                a: 255,
            },
        ), None, FontStyle::from_bits(2)),
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
                r: 139,
                g: 233,
                b: 253,
                a: 255,
            },
        ), None, FontStyle::from_bits(6)),
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
                r: 80,
                g: 250,
                b: 123,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
                r: 255,
                g: 184,
                b: 108,
                a: 255,
            },
        ), None, FontStyle::from_bits(4)),
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
                r: 255,
                g: 121,
                b: 198,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
                r: 80,
                g: 250,
                b: 123,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
                r: 139,
                g: 233,
                b: 253,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
                r: 107,
                g: 229,
                b: 253,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925375344640000,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925366754705408,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 102,
                g: 217,
                b: 239,
                a: 255,
            },
        ), None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925461255454720,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(None, None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925461268496384,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 102,
                g: 217,
                b: 239,
                a: 255,
            },
        ), None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 50102545854496768,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 248,
                g: 248,
                b: 240,
                a: 255,
            },
        ), Some(
            Color {
                r: 255,
                g: 121,
                b: 198,
                a: 255,
            },
        ), Some(FontStyle::empty())),
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
                r: 248,
                g: 248,
                b: 240,
                a: 255,
            },
        ), Some(
            Color {
                r: 189,
                g: 147,
                b: 249,
                a: 255,
            },
        ), None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46446510938914854,
                    b: 0,
                },
                Scope {
                    a: 55451420828565542,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 207,
                g: 207,
                b: 194,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46443452761636864,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46443452799385600,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 98,
                g: 114,
                b: 164,
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
                r: 255,
                g: 121,
                b: 198,
                a: 255,
            },
        ), None, None),
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
                r: 80,
                g: 250,
                b: 123,
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
                r: 230,
                g: 219,
                b: 116,
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
            ]), vec![
                ScopeStack::new_with_args(vec![], vec![
                    Scope {
                        a: 208010007789174784,
                        b: 0,
                    },
                ]),
            ]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 189,
                g: 147,
                b: 249,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130656370688,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 230,
                g: 219,
                b: 116,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 161004842024697856,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 248,
                g: 51,
                b: 51,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629323956406,
                    b: 10696049115004928,
                },
            ]), vec![
                ScopeStack::new_with_args(vec![], vec![
                    Scope {
                        a: 46446510938915183,
                        b: 10696049115004928,
                    },
                ]),
            ]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629323956395,
                    b: 10696049115004928,
                },
            ]), vec![
                ScopeStack::new_with_args(vec![], vec![
                    Scope {
                        a: 46446510938915183,
                        b: 10696049115004928,
                    },
                ]),
            ]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 238,
                g: 238,
                b: 238,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46446510938914854,
                    b: 0,
                },
                Scope {
                    a: 55451420828565542,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 139,
                g: 233,
                b: 253,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46446510938915183,
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
                r: 241,
                g: 250,
                b: 140,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46446510938915183,
                    b: 0,
                },
                Scope {
                    a: 55450570411999232,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 80,
                g: 250,
                b: 123,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46446510938915183,
                    b: 0,
                },
                Scope {
                    a: 55450570411999232,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 184,
                b: 108,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46446510938915183,
                    b: 0,
                },
                Scope {
                    a: 55450570411999232,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 121,
                b: 198,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46446510938915183,
                    b: 0,
                },
                Scope {
                    a: 55450570411999232,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 189,
                g: 147,
                b: 249,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46446510938915183,
                    b: 0,
                },
                Scope {
                    a: 55450570411999232,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 80,
                g: 250,
                b: 123,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46443371157258240,
                    b: 0,
                },
                Scope {
                    a: 46446510938915183,
                    b: 0,
                },
                Scope {
                    a: 55450570411999232,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 184,
                b: 108,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114290024639365120,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 184,
                b: 108,
                a: 255,
            },
        ), None, FontStyle::from_bits(4)),
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
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 184,
                b: 108,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
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
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 184,
                b: 108,
                a: 255,
            },
        ), None, FontStyle::from_bits(4)),
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
                r: 139,
                g: 233,
                b: 253,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629372518449,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 121,
                b: 198,
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
                r: 98,
                g: 114,
                b: 164,
                a: 255,
            },
        ), None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629374287921,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 98,
                g: 114,
                b: 164,
                a: 255,
            },
        ), Some(
            Color {
                r: 98,
                g: 114,
                b: 164,
                a: 255,
            },
        ), FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46444195790979072,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 98,
                g: 114,
                b: 164,
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
                    a: 114280120494129152,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 80,
                g: 250,
                b: 123,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114280588596215808,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 189,
                g: 147,
                b: 249,
                a: 255,
            },
        ), None, FontStyle::from_bits(2)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114280120459395072,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 207,
                g: 207,
                b: 194,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114280120459397677,
                    b: 13792273858822144,
                },
                Scope {
                    a: 844424930131968,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 248,
                g: 248,
                b: 242,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629481766961,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 49259061692923953,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 98,
                g: 114,
                b: 164,
                a: 255,
            },
        ), None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 49259061692923953,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 98,
                g: 114,
                b: 164,
                a: 255,
            },
        ), None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288788224835584,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 121,
                b: 198,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46444131406315520,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 121,
                b: 198,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288521951477942,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288521951477931,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288521949642934,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 121,
                b: 198,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288521949642923,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 121,
                b: 198,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288620745621504,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 121,
                b: 198,
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
                r: 80,
                g: 250,
                b: 123,
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
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 255,
                b: 255,
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
                r: 189,
                g: 147,
                b: 249,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130669019202,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 139,
                g: 233,
                b: 253,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130632974402,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 189,
                g: 147,
                b: 249,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925255090602050,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 48414576487038987,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 121,
                b: 198,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130643525643,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 139,
                g: 233,
                b: 253,
                a: 255,
            },
        ), None, None),
    },
]
)}
