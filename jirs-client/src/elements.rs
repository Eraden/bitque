// use syntect::easy::HighlightLines;
// use syntect::highlighting::{FontStyle, Style};
// use syntect::parsing::SyntaxReference;
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
    pub fn hi_code(&mut self, _lang: &str, code: &str) -> String {
        // let syntax = {
        //     match crate::hi::syntax_set::load().find_syntax_by_name(lang) {
        //         Some(s) => s.clone(),
        //         _ => {
        //             return code.to_string();
        //         }
        //     }
        // };
        // let mut buffer: Vec<String> = Vec::with_capacity(code.lines().count() * 2);
        // for line in code.lines() {
        //     self.hi(&syntax, line, &mut buffer);
        //     buffer.push("<br />".to_string());
        // }
        // buffer.join("")
        code.to_string()
    }

    // fn hi<'l>(&mut self, syntax: &SyntaxReference, line: &'l str, buffer: &mut Vec<String>) {
    // let mut h = HighlightLines::new(syntax, &crate::hi::THEME_SET.themes["base16-ocean-dark"]); // inspired-github
    // let tokens = { h.highlight(line, &crate::hi::syntax_set::load()) };
    //
    // for (style, token) in tokens.into_iter() {
    //     let Style {
    //         foreground: f,
    //         background: b,
    //         font_style,
    //     } = style;
    //     let fs = if font_style == FontStyle::BOLD {
    //         "font-weight: bold"
    //     } else if font_style == FontStyle::ITALIC {
    //         "font-style: italic"
    //     } else if font_style == FontStyle::UNDERLINE {
    //         "text-decoration: underline"
    //     } else {
    //         ""
    //     };
    //
    //     buffer.push(format!(
    //         r#"<span style="color: rgba({f_r}, {f_g}, {f_b}, {f_a});background:rgba({b_r}, {b_g}, {b_b}, {b_a}); {fs}">{t}</span>"#,
    //         t = if token.is_empty() { "&nbsp;" } else { token },
    //         f_r = f.r, f_g = f.g, f_b = f.b, f_a = f.a, b_r = b.r, b_g = b.g, b_b = b.b, b_a = b.a,
    //         fs = fs
    //     ));
    // }
    // }
}

pub fn define() {
    {
        let el_name = "JirsCodeView";
        let tag = "jirs-code-view";

        ElementBuilder::default()
            .identifier(el_name, tag)
            .runtime("JirsCodeBuilder")
            .body(
                r#"
            <style>
            :host { display: block; border: 1px solid black; }
            #view { background: rgba(43, 48, 59, 255); padding: 1rem; }
            #view span { white-space: pre; font-family: 'Source Code Pro', monospace; }
        </style>
        <div id='file-name'></div>
        <div id='view'></div>
        "#,
            )
            .on_connected(FillShadowElement::new(el_name, "#view", ""))
            .on_connected(
                r#"
                const lang = this.getAttribute('lang') || '';
                setTimeout(() => {{
                    const code = (this.innerHTML || '').trim();
                    console.log('connected');
                    shadow.querySelector('#view').innerHTML = runtime.hi_code(lang, code);
                }}, 1);
            "#,
            )
            .on_attr_changed("lang", r#""#)
            .on_attr_changed(
                "file-path",
                r#"
                shadow.querySelector('#file-name').innerText = newV;
            "#,
            )
            .mount();
    };
}

trait ToJs {
    fn to_js(&self) -> String;
}

impl ToJs for &str {
    fn to_js(&self) -> String {
        self.to_string()
    }
}

struct FillShadowElement {
    el_name: String,
    target: String,
    source: String,
}

impl FillShadowElement {
    pub fn new<N: Into<String>, S: Into<String>, Source: ToJs>(
        el_name: N,
        target: S,
        source: Source,
    ) -> Self {
        Self {
            el_name: el_name.into(),
            target: target.into(),
            source: source.to_js(),
        }
    }
}

impl ToJs for FillShadowElement {
    fn to_js(&self) -> String {
        let shadow = ElementBuilder::shadow_handle(&self.el_name);
        format!(
            "{shadow}.querySelector('{selector}').innerHTML = `{content}`;",
            shadow = shadow,
            selector = self.target,
            content = self.source
        )
    }
}

#[derive(Default)]
struct ElementBuilder {
    name: String,
    tag: String,
    body: String,
    runtime: String,
    on_connected: Vec<String>,
    on_attr_changed: std::collections::HashMap<String, Vec<String>>,
}

impl ToJs for ElementBuilder {
    fn to_js(&self) -> String {
        let shadow = Self::shadow_handle(&self.name);
        let runtime = Self::runtime_handle(&self.name);
        let (observe, attr_body) = if self.on_attr_changed.is_empty() {
            ("".to_string(), "".to_string())
        } else {
            let observe = self
                .on_attr_changed
                .keys()
                .map(|s| format!("'{}'", s))
                .collect::<Vec<String>>()
                .join(",");
            let mut on_changed = "switch (name) {".to_string();
            for (k, v) in self.on_attr_changed.iter() {
                let body = v.join(";");
                on_changed.push_str(
                    format!("case '{attr}': {{ {body}; break; }}", attr = k, body = body).as_str(),
                );
            }
            on_changed.push_str("}");

            (
                format!(
                    "static get observedAttributes() {{ return [{}]; }}",
                    observe
                ),
                on_changed,
            )
        };
        let on_connected: String = self.on_connected.join(";");

        format!(
            r#"
        class {name} extends HTMLElement {{
            static RUNTIME = Symbol();
            static SHADOW = Symbol();
            
            {observe}

            constructor() {{
                super();
                {shadow} = this.attachShadow({{ 'mode': 'closed' }});
                {shadow}.innerHTML = `{html}`;
            }}

            connectedCallback() {{
                const runtime = {runtime} = new JirsCodeBuilder();
                const shadow = {shadow};
                
                {on_connected}
            }}

            disconnectedCallback() {{
                {runtime}.free();
            }}

            attributeChangedCallback(name, oldV, newV) {{
                const runtime = {runtime};
                const shadow = {shadow};
                {attr_body}
            }}
        }}
        customElements.define( '{tag}', {name});
        "#,
            name = self.name,
            tag = self.tag,
            html = self.body,
            shadow = shadow,
            runtime = runtime,
            observe = observe,
            attr_body = attr_body,
            on_connected = on_connected,
        )
    }
}

impl ElementBuilder {
    pub fn identifier<N: Into<String>, T: Into<String>>(mut self, name: N, tag: T) -> Self {
        self.name = name.into();
        self.tag = tag.into();
        self
    }

    pub fn runtime<S: Into<String>>(mut self, runtime: S) -> Self {
        self.runtime = runtime.into();
        self
    }

    pub fn body<S: Into<String>>(mut self, body: S) -> Self {
        self.body = body.into();
        self
    }

    pub fn on_connected<B: ToJs>(mut self, c: B) -> Self {
        self.on_connected.push(c.to_js());
        self
    }

    pub fn on_attr_changed<N: Into<String>, R: Into<String>>(mut self, attr: N, run: R) -> Self {
        let a = attr.into();
        let r = run.into();
        self.on_attr_changed
            .entry(a)
            .or_insert_with(|| vec![])
            .push(r);
        self
    }

    pub fn shadow_handle(name: &str) -> String {
        format!("this[ {name} . SHADOW]", name = name)
    }

    pub fn runtime_handle(name: &str) -> String {
        format!("this[ {name} . RUNTIME]", name = name)
    }

    pub fn mount(&self) {
        let source = self.to_js();
        {
            use seed::*;
            log!(source);
        }
        use seed::*;
        match js_sys::eval(source.as_str()) {
            Ok(_v) => (),
            Err(e) => error!(e),
        };
    }
}
