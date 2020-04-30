use std::str::FromStr;

use crate::prop::{CssParser, ParseToken, Parser, PropertyValue, Token, ValueResult};

#[derive(Debug, PartialEq)]
pub enum ColorProperty {
    Rgba(
        PropertyValue<u8>,
        PropertyValue<u8>,
        PropertyValue<u8>,
        PropertyValue<u8>,
    ),
    Hsla(
        PropertyValue<u16>,
        PropertyValue<u8>,
        PropertyValue<u8>,
        PropertyValue<f64>,
    ),
    Current,
}

impl Token for ColorProperty {}

impl ParseToken<ColorProperty> for CssParser {
    fn parse_token(&mut self) -> ValueResult<ColorProperty> {
        self.skip_white();
        let current = self.expect_consume()?;
        let s = current.trim();

        let p = match s {
            "currentColor" => ColorProperty::Current,
            _ if s.len() == 7 && s.starts_with('#') => {
                let r = u8::from_str_radix(&s[1..=2], 16)
                    .map_err(|_| format!("invalid color {:?}", s))?;
                let g = u8::from_str_radix(&s[3..=4], 16)
                    .map_err(|_| format!("invalid color {:?}", s))?;
                let b = u8::from_str_radix(&s[5..=6], 16)
                    .map_err(|_| format!("invalid color {:?}", s))?;
                ColorProperty::Rgba(
                    PropertyValue::Other(r),
                    PropertyValue::Other(g),
                    PropertyValue::Other(b),
                    PropertyValue::Other(255),
                )
            }
            _ if s.len() == 4 && s.starts_with('#') => {
                let _x = &s[1..=1];
                let r = u8::from_str_radix(&s[1..=1].repeat(2), 16)
                    .map_err(|_| format!("invalid color {:?}", s))?;
                let g = u8::from_str_radix(&s[2..=2].repeat(2), 16)
                    .map_err(|_| format!("invalid color {:?}", s))?;
                let b = u8::from_str_radix(&s[3..=3].repeat(2), 16)
                    .map_err(|_| format!("invalid color {:?}", s))?;
                ColorProperty::Rgba(
                    PropertyValue::Other(r),
                    PropertyValue::Other(g),
                    PropertyValue::Other(b),
                    PropertyValue::Other(255),
                )
            }
            _ if s.len() == 9 && s.starts_with('#') => {
                let (r, g, b, a) = (
                    u8::from_str_radix(&s[1..=2], 16)
                        .map_err(|_| format!("invalid color {:?}", s))?,
                    u8::from_str_radix(&s[3..=4], 16)
                        .map_err(|_| format!("invalid color {:?}", s))?,
                    u8::from_str_radix(&s[5..=6], 16)
                        .map_err(|_| format!("invalid color {:?}", s))?,
                    u8::from_str_radix(&s[7..=8], 16)
                        .map_err(|_| format!("invalid color {:?}", s))?,
                );
                ColorProperty::Rgba(
                    PropertyValue::Other(r),
                    PropertyValue::Other(g),
                    PropertyValue::Other(b),
                    PropertyValue::Other(a),
                )
            }
            "rgba" => {
                self.skip_white();
                self.consume_expected("(")?;
                self.skip_white();
                let r = self.parse_token()?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let g = self.parse_token()?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let b = self.parse_token()?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let a = self.parse_token()?.into_color_alpha();
                self.skip_white();
                self.consume_expected(")")?;
                self.skip_white();
                ColorProperty::Rgba(r, g, b, a)
            }
            "rgb" => {
                self.skip_white();
                self.consume_expected("(")?;
                self.skip_white();
                let r = self.parse_token()?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let g = self.parse_token()?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let b = self.parse_token()?;
                self.skip_white();
                let a = PropertyValue::Other(255);
                self.skip_white();
                self.consume_expected(")")?;
                self.skip_white();
                ColorProperty::Rgba(r, g, b, a)
            }
            "hsla" => {
                self.skip_white();
                self.consume_expected("(")?;
                self.skip_white();
                let h = self.parse_token()?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let s = self.parse_token()?;
                self.consume_expected("%")?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let l = self.parse_token()?;
                self.consume_expected("%")?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let a = self.parse_token()?;
                match a {
                    PropertyValue::Other(f) if -0.001f64 > f || f > 1.001f64 => {
                        return Err(format!("out of range hsl alpha value {:?}", a))
                    }
                    _ => (),
                };
                self.skip_white();
                self.consume_expected(")")?;
                self.skip_white();
                ColorProperty::Hsla(h, s, l, a)
            }
            "hsl" => {
                self.skip_white();
                self.consume_expected("(")?;
                self.skip_white();
                let h = self.parse_token()?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let s = self.parse_token()?;
                self.consume_expected("%")?;
                self.skip_white();
                self.consume_expected(",")?;
                self.skip_white();
                let l = self.parse_token()?;
                self.consume_expected("%")?;
                let a = PropertyValue::Other(1f64);
                self.skip_white();
                self.consume_expected(")")?;
                self.skip_white();
                ColorProperty::Hsla(h, s, l, a)
            }

            _ => s
                .parse::<Color>()
                .map(|c| c.to_values())
                .and_then(|(r, g, b)| {
                    Ok(ColorProperty::Rgba(
                        PropertyValue::Other(r),
                        PropertyValue::Other(g),
                        PropertyValue::Other(b),
                        PropertyValue::Other(255),
                    ))
                })?,
        };
        Ok(PropertyValue::Other(p))
    }
}

pub enum Color {
    Pink,
    LightPink,
    HotPink,
    DeepPink,
    PaleVioletRed,
    MediumVioletRed,
    LightSalmon,
    Salmon,
    DarkSalmon,
    LightCoral,
    IndianRed,
    Crimson,
    Firebrick,
    DarkRed,
    Red,
    OrangeRed,
    Tomato,
    Coral,
    DarkOrange,
    Orange,
    Yellow,
    LightYellow,
    LemonChiffon,
    LightGoldenrodYellow,
    PapayaWhip,
    Moccasin,
    PeachPuff,
    PaleGoldenrod,
    Khaki,
    DarkKhaki,
    Gold,
    Cornsilk,
    BlanchedAlmond,
    Bisque,
    NavajoWhite,
    Wheat,
    Burlywood,
    Tan,
    RosyBrown,
    SandyBrown,
    Goldenrod,
    DarkGoldenrod,
    Peru,
    Chocolate,
    SaddleBrown,
    Sienna,
    Brown,
    Maroon,
    DarkOliveGreen,
    Olive,
    OliveDrab,
    YellowGreen,
    LimeGreen,
    Lime,
    LawnGreen,
    Chartreuse,
    GreenYellow,
    SpringGreen,
    MediumSpringGreen,
    LightGreen,
    PaleGreen,
    DarkSeaGreen,
    MediumAquamarine,
    MediumSeaGreen,
    SeaGreen,
    ForestGreen,
    Green,
    DarkGreen,
    Aqua,
    Cyan,
    LightCyan,
    PaleTurquoise,
    Aquamarine,
    Turquoise,
    MediumTurquoise,
    DarkTurquoise,
    LightSeaGreen,
    CadetBlue,
    DarkCyan,
    Teal,
    LightSteelBlue,
    PowderBlue,
    LightBlue,
    SkyBlue,
    LightSkyBlue,
    DeepSkyBlue,
    DodgerBlue,
    CornflowerBlue,
    SteelBlue,
    RoyalBlue,
    Blue,
    MediumBlue,
    DarkBlue,
    Navy,
    MidnightBlue,
    Lavender,
    Thistle,
    Plum,
    Violet,
    Orchid,
    Fuchsia,
    Magenta,
    MediumOrchid,
    MediumPurple,
    BlueViolet,
    DarkViolet,
    DarkOrchid,
    DarkMagenta,
    Purple,
    Indigo,
    DarkSlateBlue,
    SlateBlue,
    MediumSlateBlue,
    White,
    Snow,
    Honeydew,
    MintCream,
    Azure,
    AliceBlue,
    GhostWhite,
    WhiteSmoke,
    Seashell,
    Beige,
    OldLace,
    FloralWhite,
    Ivory,
    AntiqueWhite,
    Linen,
    LavenderBlush,
    MistyRose,
    Gainsboro,
    LightGray,
    Silver,
    DarkGray,
    Gray,
    DimGray,
    LightSlateGray,
    SlateGray,
    DarkSlateGray,
    Black,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::Pink => "Pink",
            Color::LightPink => "LightPink",
            Color::HotPink => "HotPink",
            Color::DeepPink => "DeepPink",
            Color::PaleVioletRed => "PaleVioletRed",
            Color::MediumVioletRed => "MediumVioletRed",
            Color::LightSalmon => "LightSalmon",
            Color::Salmon => "Salmon",
            Color::DarkSalmon => "DarkSalmon",
            Color::LightCoral => "LightCoral",
            Color::IndianRed => "IndianRed",
            Color::Crimson => "Crimson",
            Color::Firebrick => "Firebrick",
            Color::DarkRed => "DarkRed",
            Color::Red => "Red",
            Color::OrangeRed => "OrangeRed",
            Color::Tomato => "Tomato",
            Color::Coral => "Coral",
            Color::DarkOrange => "DarkOrange",
            Color::Orange => "Orange",
            Color::Yellow => "Yellow",
            Color::LightYellow => "LightYellow",
            Color::LemonChiffon => "LemonChiffon",
            Color::LightGoldenrodYellow => "LightGoldenrodYellow",
            Color::PapayaWhip => "PapayaWhip",
            Color::Moccasin => "Moccasin",
            Color::PeachPuff => "PeachPuff",
            Color::PaleGoldenrod => "PaleGoldenrod",
            Color::Khaki => "Khaki",
            Color::DarkKhaki => "DarkKhaki",
            Color::Gold => "Gold",
            Color::Cornsilk => "Cornsilk",
            Color::BlanchedAlmond => "BlanchedAlmond",
            Color::Bisque => "Bisque",
            Color::NavajoWhite => "NavajoWhite",
            Color::Wheat => "Wheat",
            Color::Burlywood => "Burlywood",
            Color::Tan => "Tan",
            Color::RosyBrown => "RosyBrown",
            Color::SandyBrown => "SandyBrown",
            Color::Goldenrod => "Goldenrod",
            Color::DarkGoldenrod => "DarkGoldenrod",
            Color::Peru => "Peru",
            Color::Chocolate => "Chocolate",
            Color::SaddleBrown => "SaddleBrown",
            Color::Sienna => "Sienna",
            Color::Brown => "Brown",
            Color::Maroon => "Maroon",
            Color::DarkOliveGreen => "DarkOliveGreen",
            Color::Olive => "Olive",
            Color::OliveDrab => "OliveDrab",
            Color::YellowGreen => "YellowGreen",
            Color::LimeGreen => "LimeGreen",
            Color::Lime => "Lime",
            Color::LawnGreen => "LawnGreen",
            Color::Chartreuse => "Chartreuse",
            Color::GreenYellow => "GreenYellow",
            Color::SpringGreen => "SpringGreen",
            Color::MediumSpringGreen => "MediumSpringGreen",
            Color::LightGreen => "LightGreen",
            Color::PaleGreen => "PaleGreen",
            Color::DarkSeaGreen => "DarkSeaGreen",
            Color::MediumAquamarine => "MediumAquamarine",
            Color::MediumSeaGreen => "MediumSeaGreen",
            Color::SeaGreen => "SeaGreen",
            Color::ForestGreen => "ForestGreen",
            Color::Green => "Green",
            Color::DarkGreen => "DarkGreen",
            Color::Aqua => "Aqua",
            Color::Cyan => "Cyan",
            Color::LightCyan => "LightCyan",
            Color::PaleTurquoise => "PaleTurquoise",
            Color::Aquamarine => "Aquamarine",
            Color::Turquoise => "Turquoise",
            Color::MediumTurquoise => "MediumTurquoise",
            Color::DarkTurquoise => "DarkTurquoise",
            Color::LightSeaGreen => "LightSeaGreen",
            Color::CadetBlue => "CadetBlue",
            Color::DarkCyan => "DarkCyan",
            Color::Teal => "Teal",
            Color::LightSteelBlue => "LightSteelBlue",
            Color::PowderBlue => "PowderBlue",
            Color::LightBlue => "LightBlue",
            Color::SkyBlue => "SkyBlue",
            Color::LightSkyBlue => "LightSkyBlue",
            Color::DeepSkyBlue => "DeepSkyBlue",
            Color::DodgerBlue => "DodgerBlue",
            Color::CornflowerBlue => "CornflowerBlue",
            Color::SteelBlue => "SteelBlue",
            Color::RoyalBlue => "RoyalBlue",
            Color::Blue => "Blue",
            Color::MediumBlue => "MediumBlue",
            Color::DarkBlue => "DarkBlue",
            Color::Navy => "Navy",
            Color::MidnightBlue => "MidnightBlue",
            Color::Lavender => "Lavender",
            Color::Thistle => "Thistle",
            Color::Plum => "Plum",
            Color::Violet => "Violet",
            Color::Orchid => "Orchid",
            Color::Fuchsia => "Fuchsia",
            Color::Magenta => "Magenta",
            Color::MediumOrchid => "MediumOrchid",
            Color::MediumPurple => "MediumPurple",
            Color::BlueViolet => "BlueViolet",
            Color::DarkViolet => "DarkViolet",
            Color::DarkOrchid => "DarkOrchid",
            Color::DarkMagenta => "DarkMagenta",
            Color::Purple => "Purple",
            Color::Indigo => "Indigo",
            Color::DarkSlateBlue => "DarkSlateBlue",
            Color::SlateBlue => "SlateBlue",
            Color::MediumSlateBlue => "MediumSlateBlue",
            Color::White => "White",
            Color::Snow => "Snow",
            Color::Honeydew => "Honeydew",
            Color::MintCream => "MintCream",
            Color::Azure => "Azure",
            Color::AliceBlue => "AliceBlue",
            Color::GhostWhite => "GhostWhite",
            Color::WhiteSmoke => "WhiteSmoke",
            Color::Seashell => "Seashell",
            Color::Beige => "Beige",
            Color::OldLace => "OldLace",
            Color::FloralWhite => "FloralWhite",
            Color::Ivory => "Ivory",
            Color::AntiqueWhite => "AntiqueWhite",
            Color::Linen => "Linen",
            Color::LavenderBlush => "LavenderBlush",
            Color::MistyRose => "MistyRose",
            Color::Gainsboro => "Gainsboro",
            Color::LightGray => "LightGray",
            Color::Silver => "Silver",
            Color::DarkGray => "DarkGray",
            Color::Gray => "Gray",
            Color::DimGray => "DimGray",
            Color::LightSlateGray => "LightSlateGray",
            Color::SlateGray => "SlateGray",
            Color::DarkSlateGray => "DarkSlateGray",
            Color::Black => "Black",
        }
        .to_string()
    }
}

impl Color {
    pub fn to_values(&self) -> (u8, u8, u8) {
        match self {
            Color::Pink => (255, 192, 203),
            Color::LightPink => (255, 182, 193),
            Color::HotPink => (255, 105, 180),
            Color::DeepPink => (255, 20, 147),
            Color::PaleVioletRed => (219, 112, 147),
            Color::MediumVioletRed => (199, 21, 133),
            Color::LightSalmon => (255, 160, 122),
            Color::Salmon => (250, 128, 114),
            Color::DarkSalmon => (233, 150, 122),
            Color::LightCoral => (240, 128, 128),
            Color::IndianRed => (205, 92, 92),
            Color::Crimson => (220, 20, 60),
            Color::Firebrick => (178, 34, 34),
            Color::DarkRed => (139, 0, 0),
            Color::Red => (255, 0, 0),
            Color::OrangeRed => (255, 69, 0),
            Color::Tomato => (255, 99, 71),
            Color::Coral => (255, 127, 80),
            Color::DarkOrange => (255, 140, 0),
            Color::Orange => (255, 165, 0),
            Color::Yellow => (255, 255, 0),
            Color::LightYellow => (255, 255, 224),
            Color::LemonChiffon => (255, 250, 205),
            Color::LightGoldenrodYellow => (250, 250, 210),
            Color::PapayaWhip => (255, 239, 213),
            Color::Moccasin => (255, 228, 181),
            Color::PeachPuff => (255, 218, 185),
            Color::PaleGoldenrod => (238, 232, 170),
            Color::Khaki => (240, 230, 140),
            Color::DarkKhaki => (189, 183, 107),
            Color::Gold => (255, 215, 0),
            Color::Cornsilk => (255, 248, 220),
            Color::BlanchedAlmond => (255, 235, 205),
            Color::Bisque => (255, 228, 196),
            Color::NavajoWhite => (255, 222, 173),
            Color::Wheat => (245, 222, 179),
            Color::Burlywood => (222, 184, 135),
            Color::Tan => (210, 180, 140),
            Color::RosyBrown => (188, 143, 143),
            Color::SandyBrown => (244, 164, 96),
            Color::Goldenrod => (218, 165, 32),
            Color::DarkGoldenrod => (184, 134, 11),
            Color::Peru => (205, 133, 63),
            Color::Chocolate => (210, 105, 30),
            Color::SaddleBrown => (139, 69, 19),
            Color::Sienna => (160, 82, 45),
            Color::Brown => (165, 42, 42),
            Color::Maroon => (128, 0, 0),
            Color::DarkOliveGreen => (85, 107, 47),
            Color::Olive => (128, 128, 0),
            Color::OliveDrab => (107, 142, 35),
            Color::YellowGreen => (154, 205, 50),
            Color::LimeGreen => (50, 205, 50),
            Color::Lime => (0, 255, 0),
            Color::LawnGreen => (124, 252, 0),
            Color::Chartreuse => (127, 255, 0),
            Color::GreenYellow => (173, 255, 47),
            Color::SpringGreen => (0, 255, 127),
            Color::MediumSpringGreen => (0, 250, 154),
            Color::LightGreen => (144, 238, 144),
            Color::PaleGreen => (152, 251, 152),
            Color::DarkSeaGreen => (143, 188, 143),
            Color::MediumAquamarine => (102, 205, 170),
            Color::MediumSeaGreen => (60, 179, 113),
            Color::SeaGreen => (46, 139, 87),
            Color::ForestGreen => (34, 139, 34),
            Color::Green => (0, 128, 0),
            Color::DarkGreen => (0, 100, 0),
            Color::Aqua => (0, 255, 255),
            Color::Cyan => (0, 255, 255),
            Color::LightCyan => (224, 255, 255),
            Color::PaleTurquoise => (175, 238, 238),
            Color::Aquamarine => (127, 255, 212),
            Color::Turquoise => (64, 224, 208),
            Color::MediumTurquoise => (72, 209, 204),
            Color::DarkTurquoise => (0, 206, 209),
            Color::LightSeaGreen => (32, 178, 170),
            Color::CadetBlue => (95, 158, 160),
            Color::DarkCyan => (0, 139, 139),
            Color::Teal => (0, 128, 128),
            Color::LightSteelBlue => (176, 196, 222),
            Color::PowderBlue => (176, 224, 230),
            Color::LightBlue => (173, 216, 230),
            Color::SkyBlue => (135, 206, 235),
            Color::LightSkyBlue => (135, 206, 250),
            Color::DeepSkyBlue => (0, 191, 255),
            Color::DodgerBlue => (30, 144, 255),
            Color::CornflowerBlue => (100, 149, 237),
            Color::SteelBlue => (70, 130, 180),
            Color::RoyalBlue => (65, 105, 225),
            Color::Blue => (0, 0, 255),
            Color::MediumBlue => (0, 0, 205),
            Color::DarkBlue => (0, 0, 139),
            Color::Navy => (0, 0, 128),
            Color::MidnightBlue => (25, 25, 112),
            Color::Lavender => (230, 230, 250),
            Color::Thistle => (216, 191, 216),
            Color::Plum => (221, 160, 221),
            Color::Violet => (238, 130, 238),
            Color::Orchid => (218, 112, 214),
            Color::Fuchsia => (255, 0, 255),
            Color::Magenta => (255, 0, 255),
            Color::MediumOrchid => (186, 85, 211),
            Color::MediumPurple => (147, 112, 219),
            Color::BlueViolet => (138, 43, 226),
            Color::DarkViolet => (148, 0, 211),
            Color::DarkOrchid => (153, 50, 204),
            Color::DarkMagenta => (139, 0, 139),
            Color::Purple => (128, 0, 128),
            Color::Indigo => (75, 0, 130),
            Color::DarkSlateBlue => (72, 61, 139),
            Color::SlateBlue => (106, 90, 205),
            Color::MediumSlateBlue => (123, 104, 238),
            Color::White => (255, 255, 255),
            Color::Snow => (255, 250, 250),
            Color::Honeydew => (240, 255, 240),
            Color::MintCream => (245, 255, 250),
            Color::Azure => (240, 255, 255),
            Color::AliceBlue => (240, 248, 255),
            Color::GhostWhite => (248, 248, 255),
            Color::WhiteSmoke => (245, 245, 245),
            Color::Seashell => (255, 245, 238),
            Color::Beige => (245, 245, 220),
            Color::OldLace => (253, 245, 230),
            Color::FloralWhite => (255, 250, 240),
            Color::Ivory => (255, 255, 240),
            Color::AntiqueWhite => (250, 235, 215),
            Color::Linen => (250, 240, 230),
            Color::LavenderBlush => (255, 240, 245),
            Color::MistyRose => (255, 228, 225),
            Color::Gainsboro => (220, 220, 220),
            Color::LightGray => (211, 211, 211),
            Color::Silver => (192, 192, 192),
            Color::DarkGray => (169, 169, 169),
            Color::Gray => (128, 128, 128),
            Color::DimGray => (105, 105, 105),
            Color::LightSlateGray => (119, 136, 153),
            Color::SlateGray => (112, 128, 144),
            Color::DarkSlateGray => (47, 79, 79),
            Color::Black => (0, 0, 0),
        }
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Color, Self::Err> {
        let p = match s.to_lowercase().trim() {
            "pink" | "Pink" => Color::Pink,
            "lightpink" | "LightPink" => Color::LightPink,
            "hotpink" | "HotPink" => Color::HotPink,
            "deeppink" | "DeepPink" => Color::DeepPink,
            "palevioletred" | "PaleVioletRed" => Color::PaleVioletRed,
            "mediumvioletred" | "MediumVioletRed" => Color::MediumVioletRed,
            "lightsalmon" | "LightSalmon" => Color::LightSalmon,
            "salmon" | "Salmon" => Color::Salmon,
            "darksalmon" | "DarkSalmon" => Color::DarkSalmon,
            "lightcoral" | "LightCoral" => Color::LightCoral,
            "indianred" | "IndianRed" => Color::IndianRed,
            "crimson" | "Crimson" => Color::Crimson,
            "firebrick" | "Firebrick" => Color::Firebrick,
            "darkred" | "DarkRed" => Color::DarkRed,
            "red" | "Red" => Color::Red,
            "orangered" | "OrangeRed" => Color::OrangeRed,
            "tomato" | "Tomato" => Color::Tomato,
            "coral" | "Coral" => Color::Coral,
            "darkorange" | "DarkOrange" => Color::DarkOrange,
            "orange" | "Orange" => Color::Orange,
            "yellow" | "Yellow" => Color::Yellow,
            "lightyellow" | "LightYellow" => Color::LightYellow,
            "lemonchiffon" | "LemonChiffon" => Color::LemonChiffon,
            "lightgoldenrodyellow" | "LightGoldenrodYellow" => Color::LightGoldenrodYellow,
            "papayawhip" | "PapayaWhip" => Color::PapayaWhip,
            "moccasin" | "Moccasin" => Color::Moccasin,
            "peachpuff" | "PeachPuff" => Color::PeachPuff,
            "palegoldenrod" | "PaleGoldenrod" => Color::PaleGoldenrod,
            "khaki" | "Khaki" => Color::Khaki,
            "darkkhaki" | "DarkKhaki" => Color::DarkKhaki,
            "gold" | "Gold" => Color::Gold,
            "cornsilk" | "Cornsilk" => Color::Cornsilk,
            "blanchedalmond" | "BlanchedAlmond" => Color::BlanchedAlmond,
            "bisque" | "Bisque" => Color::Bisque,
            "navajowhite" | "NavajoWhite" => Color::NavajoWhite,
            "wheat" | "Wheat" => Color::Wheat,
            "burlywood" | "Burlywood" => Color::Burlywood,
            "tan" | "Tan" => Color::Tan,
            "rosybrown" | "RosyBrown" => Color::RosyBrown,
            "sandybrown" | "SandyBrown" => Color::SandyBrown,
            "goldenrod" | "Goldenrod" => Color::Goldenrod,
            "darkgoldenrod" | "DarkGoldenrod" => Color::DarkGoldenrod,
            "peru" | "Peru" => Color::Peru,
            "chocolate" | "Chocolate" => Color::Chocolate,
            "saddlebrown" | "SaddleBrown" => Color::SaddleBrown,
            "sienna" | "Sienna" => Color::Sienna,
            "brown" | "Brown" => Color::Brown,
            "maroon" | "Maroon" => Color::Maroon,
            "darkolivegreen" | "DarkOliveGreen" => Color::DarkOliveGreen,
            "olive" | "Olive" => Color::Olive,
            "olivedrab" | "OliveDrab" => Color::OliveDrab,
            "yellowgreen" | "YellowGreen" => Color::YellowGreen,
            "limegreen" | "LimeGreen" => Color::LimeGreen,
            "lime" | "Lime" => Color::Lime,
            "lawngreen" | "LawnGreen" => Color::LawnGreen,
            "chartreuse" | "Chartreuse" => Color::Chartreuse,
            "greenyellow" | "GreenYellow" => Color::GreenYellow,
            "springgreen" | "SpringGreen" => Color::SpringGreen,
            "mediumspringgreen" | "MediumSpringGreen" => Color::MediumSpringGreen,
            "lightgreen" | "LightGreen" => Color::LightGreen,
            "palegreen" | "PaleGreen" => Color::PaleGreen,
            "darkseagreen" | "DarkSeaGreen" => Color::DarkSeaGreen,
            "mediumaquamarine" | "MediumAquamarine" => Color::MediumAquamarine,
            "mediumseagreen" | "MediumSeaGreen" => Color::MediumSeaGreen,
            "seagreen" | "SeaGreen" => Color::SeaGreen,
            "forestgreen" | "ForestGreen" => Color::ForestGreen,
            "green" | "Green" => Color::Green,
            "darkgreen" | "DarkGreen" => Color::DarkGreen,
            "aqua" | "Aqua" => Color::Aqua,
            "cyan" | "Cyan" => Color::Cyan,
            "lightcyan" | "LightCyan" => Color::LightCyan,
            "paleturquoise" | "PaleTurquoise" => Color::PaleTurquoise,
            "aquamarine" | "Aquamarine" => Color::Aquamarine,
            "turquoise" | "Turquoise" => Color::Turquoise,
            "mediumturquoise" | "MediumTurquoise" => Color::MediumTurquoise,
            "darkturquoise" | "DarkTurquoise" => Color::DarkTurquoise,
            "lightseagreen" | "LightSeaGreen" => Color::LightSeaGreen,
            "cadetblue" | "CadetBlue" => Color::CadetBlue,
            "darkcyan" | "DarkCyan" => Color::DarkCyan,
            "teal" | "Teal" => Color::Teal,
            "lightsteelblue" | "LightSteelBlue" => Color::LightSteelBlue,
            "powderblue" | "PowderBlue" => Color::PowderBlue,
            "lightblue" | "LightBlue" => Color::LightBlue,
            "skyblue" | "SkyBlue" => Color::SkyBlue,
            "lightskyblue" | "LightSkyBlue" => Color::LightSkyBlue,
            "deepskyblue" | "DeepSkyBlue" => Color::DeepSkyBlue,
            "dodgerblue" | "DodgerBlue" => Color::DodgerBlue,
            "cornflowerblue" | "CornflowerBlue" => Color::CornflowerBlue,
            "steelblue" | "SteelBlue" => Color::SteelBlue,
            "royalblue" | "RoyalBlue" => Color::RoyalBlue,
            "blue" | "Blue" => Color::Blue,
            "mediumblue" | "MediumBlue" => Color::MediumBlue,
            "darkblue" | "DarkBlue" => Color::DarkBlue,
            "navy" | "Navy" => Color::Navy,
            "midnightblue" | "MidnightBlue" => Color::MidnightBlue,
            "lavender" | "Lavender" => Color::Lavender,
            "thistle" | "Thistle" => Color::Thistle,
            "plum" | "Plum" => Color::Plum,
            "violet" | "Violet" => Color::Violet,
            "orchid" | "Orchid" => Color::Orchid,
            "fuchsia" | "Fuchsia" => Color::Fuchsia,
            "magenta" | "Magenta" => Color::Magenta,
            "mediumorchid" | "MediumOrchid" => Color::MediumOrchid,
            "mediumpurple" | "MediumPurple" => Color::MediumPurple,
            "blueviolet" | "BlueViolet" => Color::BlueViolet,
            "darkviolet" | "DarkViolet" => Color::DarkViolet,
            "darkorchid" | "DarkOrchid" => Color::DarkOrchid,
            "darkmagenta" | "DarkMagenta" => Color::DarkMagenta,
            "purple" | "Purple" => Color::Purple,
            "indigo" | "Indigo" => Color::Indigo,
            "darkslateblue" | "DarkSlateBlue" => Color::DarkSlateBlue,
            "slateblue" | "SlateBlue" => Color::SlateBlue,
            "mediumslateblue" | "MediumSlateBlue" => Color::MediumSlateBlue,
            "white" | "White" => Color::White,
            "snow" | "Snow" => Color::Snow,
            "honeydew" | "Honeydew" => Color::Honeydew,
            "mintcream" | "MintCream" => Color::MintCream,
            "azure" | "Azure" => Color::Azure,
            "aliceblue" | "AliceBlue" => Color::AliceBlue,
            "ghostwhite" | "GhostWhite" => Color::GhostWhite,
            "whitesmoke" | "WhiteSmoke" => Color::WhiteSmoke,
            "seashell" | "Seashell" => Color::Seashell,
            "beige" | "Beige" => Color::Beige,
            "oldlace" | "OldLace" => Color::OldLace,
            "floralwhite" | "FloralWhite" => Color::FloralWhite,
            "ivory" | "Ivory" => Color::Ivory,
            "antiquewhite" | "AntiqueWhite" => Color::AntiqueWhite,
            "linen" | "Linen" => Color::Linen,
            "lavenderblush" | "LavenderBlush" => Color::LavenderBlush,
            "mistyrose" | "MistyRose" => Color::MistyRose,
            "gainsboro" | "Gainsboro" => Color::Gainsboro,
            "lightgray" | "LightGray" => Color::LightGray,
            "silver" | "Silver" => Color::Silver,
            "darkgray" | "DarkGray" => Color::DarkGray,
            "gray" | "Gray" => Color::Gray,
            "dimgray" | "DimGray" => Color::DimGray,
            "lightslategray" | "LightSlateGray" => Color::LightSlateGray,
            "slategray" | "SlateGray" => Color::SlateGray,
            "darkslategray" | "DarkSlateGray" => Color::DarkSlateGray,
            "black" | "Black" => Color::Black,
            _ => return Err(format!("{:?} is not a predefined color", s)),
        };
        Ok(p)
    }
}
