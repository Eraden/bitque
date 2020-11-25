use jirs_syntect::*;
use jirs_syntect::highlighting::*;
pub fn load() -> Theme {
Theme::new(  Some(
    "Dark Neon",
),
  None,
  ThemeSettings {
    foreground: Some(
        Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        },
    ),
    background: Some(
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        },
    ),
    caret: Some(
        Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        },
    ),
    line_highlight: Some(
        Color {
            r: 255,
            g: 255,
            b: 255,
            a: 36,
        },
    ),
    misspelling: None,
    minimap_border: None,
    accent: None,
    popup_css: None,
    phantom_css: None,
    bracket_contents_foreground: None,
    bracket_contents_options: None,
    brackets_foreground: None,
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
            r: 51,
            g: 51,
            b: 51,
            a: 255,
        },
    ),
    selection_foreground: None,
    selection_background: None,
    selection_border: None,
    inactive_selection: None,
    inactive_selection_foreground: None,
    guide: None,
    active_guide: None,
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
                r: 124,
                g: 124,
                b: 124,
                a: 255,
            },
        ), Some(
            Color {
                r: 33,
                g: 33,
                b: 33,
                a: 255,
            },
        ), Some(FontStyle::empty())),
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
                r: 248,
                g: 248,
                b: 248,
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
                r: 102,
                g: 204,
                b: 255,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
                r: 102,
                g: 204,
                b: 255,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636628098744320,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 170,
                g: 170,
                b: 170,
                a: 255,
            },
        ), None, None),
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
                r: 255,
                g: 255,
                b: 182,
                a: 255,
            },
        ), None, FontStyle::from_bits(2)),
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
                r: 255,
                g: 255,
                b: 182,
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
                r: 207,
                g: 203,
                b: 144,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 48414439023575040,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 102,
                g: 204,
                b: 255,
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
                r: 153,
                g: 204,
                b: 153,
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
                r: 204,
                g: 255,
                b: 102,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
                r: 255,
                g: 115,
                b: 253,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47287796087390208,
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
                    a: 49258120924364800,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 198,
                g: 197,
                b: 254,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
                r: 253,
                g: 95,
                b: 241,
                a: 255,
            },
        ), None, FontStyle::from_bits(6)),
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
                r: 253,
                g: 95,
                b: 241,
                a: 255,
            },
        ), Some(
            Color {
                r: 86,
                g: 45,
                b: 86,
                a: 191,
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
        ]),
        style: StyleModifier::new(None, Some(
            Color {
                r: 177,
                g: 179,
                b: 186,
                a: 8,
            },
        ), Some(FontStyle::empty())),
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
                r: 155,
                g: 92,
                b: 46,
                a: 255,
            },
        ), None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844424930131968,
                    b: 0,
                },
                Scope {
                    a: 55450570411999232,
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
                r: 237,
                g: 237,
                b: 237,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844424930131968,
                    b: 0,
                },
                Scope {
                    a: 55450570411999232,
                    b: 0,
                },
                Scope {
                    a: 844424930131968,
                    b: 0,
                },
                Scope {
                    a: 47288521949642752,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 0,
                g: 160,
                b: 160,
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
                Scope {
                    a: 59954170039369728,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 0,
                g: 160,
                b: 160,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
                r: 233,
                g: 192,
                b: 98,
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
                Scope {
                    a: 59955200847314944,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55450759390560256,
                    b: 0,
                },
                Scope {
                    a: 844708410753024,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55450759390560256,
                    b: 0,
                },
                Scope {
                    a: 55450759530545152,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 128,
                b: 0,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55450759398096896,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 198,
                g: 162,
                b: 79,
                a: 255,
            },
        ), Some(
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 15,
            },
        ), Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55450759445348352,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 177,
                g: 138,
                b: 61,
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
                Scope {
                    a: 49258120924364800,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 138,
                g: 154,
                b: 149,
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
                r: 218,
                g: 208,
                b: 133,
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
                r: 255,
                g: 210,
                b: 167,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46444466374770688,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 137,
                g: 150,
                b: 168,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46444466374770688,
                    b: 0,
                },
                Scope {
                    a: 52635820644892672,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 175,
                g: 196,
                b: 219,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46444986064961536,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 103,
                g: 103,
                b: 103,
                a: 255,
            },
        ), None, FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46446583793123328,
                    b: 0,
                },
                Scope {
                    a: 46446635332403200,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46446583793123328,
                    b: 0,
                },
                Scope {
                    a: 46446635332403200,
                    b: 0,
                },
                Scope {
                    a: 59391220085948416,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46446583793123328,
                    b: 0,
                },
                Scope {
                    a: 46446635332403200,
                    b: 0,
                },
                Scope {
                    a: 55450570411999232,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46454533777260544,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46454533777260544,
                    b: 0,
                },
                Scope {
                    a: 59391220085948416,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46454533777260544,
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
                r: 73,
                g: 73,
                b: 73,
                a: 255,
            },
        ), None, None),
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
                r: 102,
                g: 204,
                b: 255,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844424930131968,
                    b: 0,
                },
                Scope {
                    a: 59392130632122368,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844424930131968,
                    b: 0,
                },
                Scope {
                    a: 59392186477182976,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46444230200066048,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46444230200066048,
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
                r: 102,
                g: 204,
                b: 255,
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
                r: 170,
                g: 170,
                b: 170,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130632122742,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392186477183350,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 225,
                g: 137,
                b: 100,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
                r: 102,
                g: 204,
                b: 255,
                a: 255,
            },
        ), None, FontStyle::from_bits(2)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46445565886464000,
                    b: 0,
                },
                Scope {
                    a: 59392186477183176,
                    b: 142426338215591936,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 143,
                g: 157,
                b: 106,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46445565886464000,
                    b: 0,
                },
                Scope {
                    a: 59392186477183489,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 139,
                g: 152,
                b: 171,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
                r: 98,
                g: 177,
                b: 254,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
                r: 237,
                g: 237,
                b: 237,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46454538072227840,
                    b: 0,
                },
                Scope {
                    a: 61925409737015310,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46445510050971648,
                    b: 0,
                },
                Scope {
                    a: 61925409737015310,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 249,
                g: 238,
                b: 152,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
                r: 134,
                g: 147,
                b: 165,
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
                r: 135,
                g: 195,
                b: 138,
                a: 255,
            },
        ), None, Some(FontStyle::empty())),
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
                r: 143,
                g: 157,
                b: 106,
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
                r: 248,
                g: 248,
                b: 248,
                a: 255,
            },
        ), Some(
            Color {
                r: 14,
                g: 34,
                b: 49,
                a: 255,
            },
        ), FontStyle::from_bits(4)),
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
                r: 248,
                g: 248,
                b: 248,
                a: 255,
            },
        ), Some(
            Color {
                r: 66,
                g: 14,
                b: 9,
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
                r: 248,
                g: 248,
                b: 248,
                a: 255,
            },
        ), Some(
            Color {
                r: 74,
                g: 65,
                b: 13,
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
                r: 248,
                g: 248,
                b: 248,
                a: 255,
            },
        ), Some(
            Color {
                r: 37,
                g: 59,
                b: 34,
                a: 255,
            },
        ), None),
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
                r: 233,
                g: 192,
                b: 98,
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
                r: 233,
                g: 192,
                b: 98,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
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
                r: 225,
                g: 137,
                b: 100,
                a: 255,
            },
        ), None, FontStyle::from_bits(2)),
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
                r: 225,
                g: 212,
                b: 185,
                a: 255,
            },
        ), Some(
            Color {
                r: 254,
                g: 224,
                b: 156,
                a: 18,
            },
        ), FontStyle::from_bits(4)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114281636568236032,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114281636568236032,
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
                r: 254,
                g: 220,
                b: 197,
                a: 255,
            },
        ), Some(
            Color {
                r: 99,
                g: 45,
                b: 4,
                a: 255,
            },
        ), Some(FontStyle::empty())),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114280017365565440,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 225,
                g: 212,
                b: 185,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114280120444780544,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 87,
                g: 139,
                b: 179,
                a: 255,
            },
        ), Some(
            Color {
                r: 177,
                g: 179,
                b: 186,
                a: 8,
            },
        ), Some(FontStyle::empty())),
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
                r: 96,
                g: 166,
                b: 51,
                a: 255,
            },
        ), Some(
            Color {
                r: 36,
                g: 36,
                b: 36,
                a: 255,
            },
        ), None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46444161487014442,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46444161513753130,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(None, Some(
            Color {
                r: 238,
                g: 238,
                b: 238,
                a: 41,
            },
        ), None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46444161448872490,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(None, Some(
            Color {
                r: 117,
                g: 16,
                b: 18,
                a: 255,
            },
        ), None),
    },
]
)}
