use jirs_syntect::*;
use jirs_syntect::highlighting::*;
pub fn load() -> Theme {
Theme::new(  Some(
    "gruvbox",
),
  Some(
    "Martin Radimec",
),
  ThemeSettings {
    foreground: Some(
        Color {
            r: 40,
            g: 40,
            b: 40,
            a: 170,
        },
    ),
    background: Some(
        Color {
            r: 252,
            g: 240,
            b: 202,
            a: 255,
        },
    ),
    caret: Some(
        Color {
            r: 60,
            g: 56,
            b: 54,
            a: 255,
        },
    ),
    line_highlight: Some(
        Color {
            r: 237,
            g: 218,
            b: 181,
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
            r: 146,
            g: 131,
            b: 116,
            a: 255,
        },
    ),
    bracket_contents_options: None,
    brackets_foreground: Some(
        Color {
            r: 213,
            g: 196,
            b: 161,
            a: 255,
        },
    ),
    brackets_background: None,
    brackets_options: None,
    tags_foreground: None,
    tags_options: None,
    highlight: None,
    find_highlight: None,
    find_highlight_foreground: None,
    gutter: None,
    gutter_foreground: None,
    selection: Some(
        Color {
            r: 214,
            g: 195,
            b: 163,
            a: 255,
        },
    ),
    selection_foreground: None,
    selection_background: None,
    selection_border: None,
    inactive_selection: None,
    inactive_selection_foreground: None,
    guide: Some(
        Color {
            r: 237,
            g: 218,
            b: 181,
            a: 255,
        },
    ),
    active_guide: Some(
        Color {
            r: 124,
            g: 111,
            b: 100,
            a: 255,
        },
    ),
    stack_guide: Some(
        Color {
            r: 190,
            g: 173,
            b: 149,
            a: 255,
        },
    ),
    highlight_foreground: None,
    shadow: None,
},
  vec![
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
                r: 7,
                g: 102,
                b: 120,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629324873728,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 63,
                b: 113,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59954170039369728,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 63,
                b: 113,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
                r: 121,
                g: 116,
                b: 14,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
                r: 40,
                g: 40,
                b: 40,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59391220085948416,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 64,
                g: 121,
                b: 89,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636628119257088,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636628098744320,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636628115390464,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636628111654912,
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
                    a: 52636628112048128,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636628118077440,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636628114800640,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 178,
                g: 60,
                b: 21,
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
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636628113752064,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636787012534272,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636636688678912,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 157,
                g: 0,
                b: 6,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
                r: 157,
                g: 0,
                b: 6,
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
            ]), vec![
                ScopeStack::new_with_args(vec![], vec![
                    Scope {
                        a: 55451949269319680,
                        b: 0,
                    },
                ]),
                ScopeStack::new_with_args(vec![], vec![
                    Scope {
                        a: 55451949170753536,
                        b: 0,
                    },
                ]),
            ]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55451949170753536,
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
                r: 121,
                g: 116,
                b: 14,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
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
                r: 146,
                g: 131,
                b: 116,
                a: 255,
            },
        ), None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55450759390560256,
                    b: 0,
                },
                Scope {
                    a: 59955200847314944,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 121,
                g: 116,
                b: 14,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61924494876344320,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 181,
                g: 118,
                b: 20,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
        style: StyleModifier::new(Some(
            Color {
                r: 40,
                g: 40,
                b: 40,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
                r: 40,
                g: 40,
                b: 40,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46444882985746432,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 40,
                g: 40,
                b: 40,
                a: 255,
            },
        ), None, None),
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
                r: 40,
                g: 40,
                b: 40,
                a: 255,
            },
        ), Some(
            Color {
                r: 147,
                g: 43,
                b: 30,
                a: 255,
            },
        ), None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 281474976710656,
                    b: 0,
                },
                Scope {
                    a: 844424930131968,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55451949170753536,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844424930131968,
                    b: 0,
                },
                Scope {
                    a: 844424930131968,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 40,
                g: 40,
                b: 40,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55451420815523840,
                    b: 0,
                },
                Scope {
                    a: 844424930131968,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 121,
                g: 116,
                b: 14,
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
                r: 121,
                g: 116,
                b: 14,
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
                r: 181,
                g: 118,
                b: 20,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
                r: 64,
                g: 121,
                b: 89,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
                r: 64,
                g: 121,
                b: 89,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46444230150717440,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46444230150717440,
                    b: 0,
                },
                Scope {
                    a: 59391220085948416,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 64,
                g: 121,
                b: 89,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955136441680405,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 7,
                g: 102,
                b: 120,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46445565886464000,
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
                r: 157,
                g: 0,
                b: 6,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46445565886464000,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392186477183489,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 121,
                g: 116,
                b: 14,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46445565886464000,
                    b: 0,
                },
                Scope {
                    a: 59392186477183179,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 121,
                g: 116,
                b: 14,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925375377670158,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 64,
                g: 121,
                b: 89,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46444466405638144,
                    b: 0,
                },
                Scope {
                    a: 52636636720398336,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 181,
                g: 118,
                b: 20,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46445510050971648,
                    b: 0,
                },
                Scope {
                    a: 59954170039369728,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 181,
                g: 118,
                b: 20,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46445510050971648,
                    b: 0,
                },
                Scope {
                    a: 61925409874837518,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 178,
                g: 60,
                b: 21,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46444934549078030,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 181,
                g: 118,
                b: 20,
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
                r: 40,
                g: 40,
                b: 40,
                a: 255,
            },
        ), Some(
            Color {
                r: 7,
                g: 102,
                b: 120,
                a: 255,
            },
        ), None),
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
                r: 40,
                g: 40,
                b: 40,
                a: 255,
            },
        ), Some(
            Color {
                r: 157,
                g: 0,
                b: 6,
                a: 255,
            },
        ), None),
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
                r: 40,
                g: 40,
                b: 40,
                a: 255,
            },
        ), Some(
            Color {
                r: 181,
                g: 118,
                b: 20,
                a: 255,
            },
        ), None),
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
                r: 40,
                g: 40,
                b: 40,
                a: 255,
            },
        ), Some(
            Color {
                r: 64,
                g: 121,
                b: 89,
                a: 255,
            },
        ), None),
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
                    a: 114281636568236032,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 64,
                g: 121,
                b: 89,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130632450251,
                    b: 16325548649218048,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 64,
                g: 121,
                b: 89,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636787086327808,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 146,
                g: 131,
                b: 116,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955089163288576,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636787033309198,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 63,
                b: 113,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629324873742,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 121,
                g: 116,
                b: 14,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 49259061525020672,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 181,
                g: 118,
                b: 20,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55451949119635499,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 40,
                g: 40,
                b: 40,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955136498106437,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 121,
                g: 116,
                b: 14,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955136498040901,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 121,
                g: 116,
                b: 14,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 48414576635413983,
                    b: 742812463539421184,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 742537444071505920,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 64,
                g: 121,
                b: 89,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 36310387959791616,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 181,
                g: 118,
                b: 20,
                a: 255,
            },
        ), None, FontStyle::from_bits(2)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 36312393709518848,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 121,
                g: 116,
                b: 14,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 36312075881938944,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 63,
                b: 113,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 36321610709336064,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 64,
                g: 121,
                b: 89,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 36321615004303360,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 181,
                g: 118,
                b: 20,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 36321619299270656,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 181,
                g: 118,
                b: 20,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 36320725946073088,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 181,
                g: 118,
                b: 20,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 36320726119284736,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 181,
                g: 118,
                b: 20,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 36321627889205248,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 157,
                g: 0,
                b: 6,
                a: 255,
            },
        ), None, None),
    },
]
)}
