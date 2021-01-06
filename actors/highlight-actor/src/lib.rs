use jirs_data::HighlightedCode;
use {
    actix::{Actor, Handler, SyncContext},
    std::sync::Arc,
    syntect::{
        easy::HighlightLines,
        highlighting::{Style, ThemeSet},
        parsing::SyntaxSet,
    },
};

mod load;

lazy_static::lazy_static! {
    pub static ref THEME_SET: Arc<ThemeSet> = Arc::new(load::integrated_themeset());
    pub static ref SYNTAX_SET: Arc<SyntaxSet> = Arc::new(load::integrated_syntaxset());
}

#[derive(Debug)]
pub enum HighlightError {
    UnknownLanguage,
    UnknownTheme,
    ResultUnserializable,
}

fn hi<'l>(code: &'l str, lang: &'l str) -> Result<Vec<(Style, &'l str)>, HighlightError> {
    let set = SYNTAX_SET
        .as_ref()
        .find_syntax_by_name(lang)
        .ok_or_else(|| HighlightError::UnknownLanguage)?;
    let theme: &syntect::highlighting::Theme = THEME_SET
        .as_ref()
        .themes
        .get("GitHub")
        .ok_or_else(|| HighlightError::UnknownTheme)?;

    let mut hi = HighlightLines::new(set, theme);
    Ok(hi.highlight(code, SYNTAX_SET.as_ref()))
}

#[derive(Debug, Default)]
pub struct HighlightActor {}

impl Actor for HighlightActor {
    type Context = SyncContext<Self>;
}

#[derive(actix::Message)]
#[rtype(result = "Result<HighlightedCode, HighlightError>")]
pub struct HighlightCode {
    pub code: String,
    pub lang: String,
}

impl Handler<HighlightCode> for HighlightActor {
    type Result = Result<HighlightedCode, HighlightError>;

    fn handle(&mut self, msg: HighlightCode, _ctx: &mut Self::Context) -> Self::Result {
        let res: Vec<(Style, &str)> = hi(&msg.code, &msg.lang)?;

        Ok(HighlightedCode {
            parts: res
                .into_iter()
                .map(|(style, part)| {
                    (
                        jirs_data::Style {
                            foreground: jirs_data::Color {
                                r: style.foreground.r,
                                g: style.foreground.g,
                                b: style.foreground.b,
                                a: style.foreground.a,
                            },
                            background: jirs_data::Color {
                                r: style.background.r,
                                g: style.background.g,
                                b: style.background.b,
                                a: style.background.a,
                            },
                            font_style: style.font_style.bits(),
                        },
                        part.to_string(),
                    )
                })
                .collect(),
        })
    }
}

#[derive(actix::Message)]
#[rtype(result = "Result<String, HighlightError>")]
pub struct TextHighlightCode {
    pub code: String,
    pub lang: String,
}

impl Handler<TextHighlightCode> for HighlightActor {
    type Result = Result<String, HighlightError>;

    fn handle(&mut self, msg: TextHighlightCode, ctx: &mut Self::Context) -> Self::Result {
        let v = self.handle(
            HighlightCode {
                lang: msg.lang,
                code: msg.code,
            },
            ctx,
        )?;
        Ok(v.parts
            .into_iter()
            .fold(String::new(), |mem, (style, text)| {
                format!(
                    "{mem}<span style=\"color:rgba({fr},{fg},{fb},{fa});background:rgba({br},{bg},{bb},{ba})\">{txt}</span>",
                    mem = mem,
                    fr = style.foreground.r, fg = style.foreground.g, fb = style.foreground.b, fa = style.foreground.a,
                    br = style.background.r, bg = style.background.g, bb = style.background.b, ba = style.background.a,
                    txt = text
                )
            }))
    }
}
