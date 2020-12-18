use {
    crate::errors::{HighlightError, ServiceError},
    actix::{Actor, Handler, SyncContext},
    serde::{Deserialize, Serialize},
    std::fs::*,
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

fn hi<'l>(code: &'l str, lang: &'l str) -> Result<Vec<(Style, &'l str)>, ServiceError> {
    let set = SYNTAX_SET
        .as_ref()
        .find_syntax_by_name(lang)
        .ok_or_else(|| ServiceError::Highlight(HighlightError::UnknownLanguage))?;
    let theme: &syntect::highlighting::Theme = THEME_SET
        .as_ref()
        .themes
        .get("GitHub")
        .ok_or_else(|| ServiceError::Highlight(HighlightError::UnknownTheme))?;

    let mut hi = HighlightLines::new(set, theme);
    Ok(hi.highlight(code, SYNTAX_SET.as_ref()))
}

#[derive(Debug, Default)]
pub struct HighlightActor {}

impl Actor for HighlightActor {
    type Context = SyncContext<Self>;
}

#[derive(actix::Message)]
#[rtype(result = "Result<Vec<u8>, ServiceError>")]
pub struct HighlightCode {
    pub code: String,
    pub lang: String,
}

impl Handler<HighlightCode> for HighlightActor {
    type Result = Result<Vec<u8>, ServiceError>;

    fn handle(&mut self, msg: HighlightCode, _ctx: &mut Self::Context) -> Self::Result {
        let res = hi(&msg.code, &msg.lang)?;
        bincode::serialize(&res)
            .map_err(|_| ServiceError::Highlight(HighlightError::ResultUnserializable))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub port: usize,
    pub bind: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            port: std::env::var("HI_PORT")
                .map_err(|_| ())
                .and_then(|s| s.parse().map_err(|_| ()))
                .unwrap_or_else(|_| 6541),
            bind: std::env::var("HI_BIND").unwrap_or_else(|_| "0.0.0.0".to_string()),
        }
    }
}

impl Configuration {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.bind, self.port)
    }

    pub fn read() -> Self {
        let contents: String = read_to_string(Self::config_file()).unwrap_or_default();
        match toml::from_str(contents.as_str()) {
            Ok(config) => config,
            _ => {
                let config = Configuration::default();
                config.write().unwrap_or_else(|e| panic!(e));
                config
            }
        }
    }

    pub fn write(&self) -> Result<(), String> {
        let s = toml::to_string(self).map_err(|e| e.to_string())?;
        write(Self::config_file(), s.as_str()).map_err(|e| e.to_string())?;
        Ok(())
    }

    #[cfg(not(test))]
    pub fn config_file() -> &'static str {
        "highlight.toml"
    }

    #[cfg(test)]
    pub fn config_file() -> &'static str {
        "highlight.test.toml"
    }
}
