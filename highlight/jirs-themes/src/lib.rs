use jirs_syntect::highlighting::*;
#[cfg(feature = "1337")]
pub mod t0;
#[cfg(feature = "Dark Neon")]
pub mod t1;
#[cfg(feature = "Dracula")]
pub mod t2;
#[cfg(feature = "GitHub")]
pub mod t3;
#[cfg(feature = "Monokai Extended")]
pub mod t4;
#[cfg(feature = "Monokai Extended Bright")]
pub mod t5;
#[cfg(feature = "Monokai Extended Light")]
pub mod t6;
#[cfg(feature = "Monokai Extended Origin")]
pub mod t7;
#[cfg(feature = "Nord")]
pub mod t8;
#[cfg(feature = "OneHalfLight")]
pub mod t9;
#[cfg(feature = "OneHalfLight")]
pub mod t10;
#[cfg(feature = "Solarized (dark)")]
pub mod t11;
#[cfg(feature = "Solarized (light)")]
pub mod t12;
#[cfg(feature = "Sublime Snazzy")]
pub mod t13;
#[cfg(feature = "TwoDark")]
pub mod t14;
#[cfg(feature = "ANSI Dark")]
pub mod t15;
#[cfg(feature = "ANSI Light")]
pub mod t16;
#[cfg(feature = "Base16")]
pub mod t17;
#[cfg(feature = "Base16 256")]
pub mod t18;
#[cfg(feature = "gruvbox")]
pub mod t19;
#[cfg(feature = "gruvbox")]
pub mod t20;
#[cfg(feature = "gruvbox")]
pub mod t21;
#[cfg(feature = "zenburn")]
pub mod t22;

pub fn load() -> ThemeSet {
  let mut ts = ThemeSet::new();
  #[cfg(feature = "1337")]
  ts.themes.insert("1337".to_string(), t0::load());
  #[cfg(feature = "Dark Neon")]
  ts.themes.insert("DarkNeon".to_string(), t1::load());
  #[cfg(feature = "Dracula")]
  ts.themes.insert("Dracula".to_string(), t2::load());
  #[cfg(feature = "GitHub")]
  ts.themes.insert("GitHub".to_string(), t3::load());
  #[cfg(feature = "Monokai Extended")]
  ts.themes.insert("Monokai Extended".to_string(), t4::load());
  #[cfg(feature = "Monokai Extended Bright")]
  ts.themes.insert("Monokai Extended Bright".to_string(), t5::load());
  #[cfg(feature = "Monokai Extended Light")]
  ts.themes.insert("Monokai Extended Light".to_string(), t6::load());
  #[cfg(feature = "Monokai Extended Origin")]
  ts.themes.insert("Monokai Extended Origin".to_string(), t7::load());
  #[cfg(feature = "Nord")]
  ts.themes.insert("Nord".to_string(), t8::load());
  #[cfg(feature = "OneHalfLight")]
  ts.themes.insert("OneHalfDark".to_string(), t9::load());
  #[cfg(feature = "OneHalfLight")]
  ts.themes.insert("OneHalfLight".to_string(), t10::load());
  #[cfg(feature = "Solarized (dark)")]
  ts.themes.insert("Solarized (dark)".to_string(), t11::load());
  #[cfg(feature = "Solarized (light)")]
  ts.themes.insert("Solarized (light)".to_string(), t12::load());
  #[cfg(feature = "Sublime Snazzy")]
  ts.themes.insert("Sublime Snazzy".to_string(), t13::load());
  #[cfg(feature = "TwoDark")]
  ts.themes.insert("TwoDark".to_string(), t14::load());
  #[cfg(feature = "ANSI Dark")]
  ts.themes.insert("ansi-dark".to_string(), t15::load());
  #[cfg(feature = "ANSI Light")]
  ts.themes.insert("ansi-light".to_string(), t16::load());
  #[cfg(feature = "Base16")]
  ts.themes.insert("base16".to_string(), t17::load());
  #[cfg(feature = "Base16 256")]
  ts.themes.insert("base16-256".to_string(), t18::load());
  #[cfg(feature = "gruvbox")]
  ts.themes.insert("gruvbox".to_string(), t19::load());
  #[cfg(feature = "gruvbox")]
  ts.themes.insert("gruvbox-light".to_string(), t20::load());
  #[cfg(feature = "gruvbox")]
  ts.themes.insert("gruvbox-white".to_string(), t21::load());
  #[cfg(feature = "zenburn")]
  ts.themes.insert("zenburn".to_string(), t22::load());
  ts
}
