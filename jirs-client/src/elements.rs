use syntect::easy::HighlightLines;
use syntect::highlighting::{FontStyle, Style};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(final, js_name = JirsCodeBuilder)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JirsCodeBuilder {}

#[wasm_bindgen]
impl JirsCodeBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[wasm_bindgen]
    pub fn hi(&mut self, lang: &str, line: &str) -> String {
        let syntax = match crate::hi::SYNTAX_SET.find_syntax_by_name(lang) {
            Some(s) => s,
            _ => {
                return line.to_string();
            }
        };
        let mut h = HighlightLines::new(syntax, &crate::hi::THEME_SET.themes["base16-ocean-dark"]); // inspired-github
        let tokens = h.highlight(line, &crate::hi::SYNTAX_SET);

        let parts: Vec<String> = tokens
            .into_iter()
            .map(|(style, token)| {
                let Style {
                    foreground: f,
                    background: b,
                    font_style,
                } = style;
                let fs = if font_style == FontStyle::BOLD {
                    "font-weight: bold"
                } else if font_style == FontStyle::ITALIC {
                    "font-style: italic"
                } else {
                    "font-decoration: underline"
                };
                let f = format!("rgba({}, {}, {}, {})", f.r, f.g, f.b, f.a);
                let b = format!("rgba({}, {}, {}, {})", b.r, b.g, b.b, b.a);
                format!(
                    r#"<span style="color: {f};background:{b}; {fs}">{t}</span>"#,
                    t = if token.is_empty() { "&nbsp;" } else { token },
                    f = f,
                    b = b,
                    fs = fs
                )
            })
            .collect();
        parts.join("")
    }
}

pub fn define() {
    create_custom_element(
        "jirs-code-view",
        "JirsCodeView",
        r#"
        <style>
            :host { display: block; border: 1px solid black; background: rgba(43, 48, 59, 255); padding: 1rem; }
            :host { margin-left: 400px; }
            #view span { white-space: pre; }
        </style>
        <div id='view'></div>
    "#,
    );
}

fn create_custom_element(tag: &str, name: &str, html: &str) {
    let source = format!(
        r#"
        class {name} extends HTMLElement {{
            static RUNTIME = Symbol();
            static SHADOW = Symbol();
            
            static get observedAttributes() {{ return ['lang']; }}

            constructor() {{
                super();
                this[ {name} . SHADOW] = this.attachShadow({{ 'mode': 'closed' }});
                this[ {name} . SHADOW].innerHTML = `{html}`;
            }}

            connectedCallback() {{
                this[ {name} . RUNTIME] = new JirsCodeBuilder();
                const view = this[ {name} . SHADOW].querySelector('#view');
                view.innerHTML = '';
                const lang = this.getAttribute('lang') || '';
                
                setTimeout(() => {{
                    const hi = () => {{
                        const line = code.shift();
                        if (line === undefined) return;
                        const s = this[ {name} . RUNTIME].hi(lang, line);
                        view.innerHTML += `${{s}}<br />`;
                        setTimeout(() => hi(), 10);
                    }};
                    hi();
                }}, 10);
            }}

            disconnectedCallback() {{
                this[ {name} . RUNTIME].free();
            }}

            attributeChangedCallback(name, oldV, newV) {{
            }}
        }}
        customElements.define( '{tag}', {name});
        "#,
        name = name,
        tag = tag,
        html = html,
    );
    {
        use seed::*;
        match js_sys::eval(source.as_str()) {
            Ok(_v) => (),
            Err(e) => error!(e),
        };
    };
}
