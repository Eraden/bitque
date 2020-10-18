use seed::{prelude::*, *};

use crate::shared::styled_button::StyledButton;
use crate::shared::styled_icon::{Icon, StyledIcon};
use crate::shared::styled_select::{StyledSelect, StyledSelectState};
use crate::shared::styled_tooltip::StyledTooltip;
use crate::shared::{ToChild, ToNode};
use crate::{FieldId, Msg, RteField};

#[derive(Debug, Clone, Copy)]
pub enum HeadingSize {
    Normal,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl HeadingSize {
    fn all() -> Vec<Self> {
        use HeadingSize::*;

        vec![Normal, H1, H2, H3, H4, H5, H6]
    }
}

impl HeadingSize {
    fn as_str<'l>(&self) -> &'l str {
        use HeadingSize::*;

        match self {
            Normal => "Normal",
            H1 => "H1",
            H2 => "H2",
            H3 => "H3",
            H4 => "H4",
            H5 => "H5",
            H6 => "H6",
        }
    }
}

#[derive(Debug)]
pub enum RteIndentMsg {
    Increase,
    Decrease,
}

#[derive(Debug)]
pub enum RteMsg {
    Bold,
    Italic,
    Underscore,
    Undo,
    Redo,
    Strikethrough,
    Copy,
    Paste,
    Cut,
    JustifyFull,
    JustifyCenter,
    JustifyLeft,
    JustifyRight,
    InsertParagraph,
    InsertHeading(HeadingSize),
    InsertUnorderedList,
    InsertOrderedList,
    RemoveFormat,
    Subscript,
    Superscript,

    // table
    TableSetVisibility(bool),
    TableSetRows(u16),
    TableSetColumns(u16),
    InsertTable { rows: u16, cols: u16 },
    ChangeIndent(RteIndentMsg),

    // code
    InsertCode(bool),
    CodeChanged(String),
    InjectCode,

    RequestFocus(uuid::Uuid),
}

#[derive(Debug)]
pub struct ExecCommand<'l> {
    pub(crate) name: &'l str,
    pub(crate) param: &'l str,
}

impl<'l> ExecCommand<'l> {
    pub fn new(name: &'l str) -> Self {
        Self::new_with_param(name, "")
    }

    pub fn new_with_param(name: &'l str, param: &'l str) -> Self {
        Self { name, param }
    }
}

impl RteMsg {
    pub fn to_command(&self) -> Option<ExecCommand> {
        match self {
            RteMsg::Bold => Some(ExecCommand::new("bold")),
            RteMsg::Italic => Some(ExecCommand::new("italic")),
            RteMsg::Underscore => Some(ExecCommand::new("underline")),
            RteMsg::Undo => Some(ExecCommand::new("undo")),
            RteMsg::Redo => Some(ExecCommand::new("redo")),
            RteMsg::Strikethrough => Some(ExecCommand::new("strikeThrough")),
            RteMsg::Copy => Some(ExecCommand::new("copy")),
            RteMsg::Paste => Some(ExecCommand::new("paste")),
            RteMsg::Cut => Some(ExecCommand::new("cut")),
            RteMsg::JustifyFull => Some(ExecCommand::new("justifyFull")),
            RteMsg::JustifyCenter => Some(ExecCommand::new("justifyCenter")),
            RteMsg::JustifyLeft => Some(ExecCommand::new("justifyLeft")),
            RteMsg::JustifyRight => Some(ExecCommand::new("justifyRight")),
            RteMsg::InsertParagraph => Some(ExecCommand::new("insertParagraph")),
            RteMsg::InsertHeading(heading) => match heading {
                HeadingSize::H1
                | HeadingSize::H2
                | HeadingSize::H3
                | HeadingSize::H4
                | HeadingSize::H5
                | HeadingSize::H6 => Some(ExecCommand::new_with_param("heading", heading.as_str())),
                HeadingSize::Normal => Some(ExecCommand::new_with_param("formatBlock", "div")),
            },
            RteMsg::InsertUnorderedList => Some(ExecCommand::new("insertUnorderedList")),
            RteMsg::InsertOrderedList => Some(ExecCommand::new("insertOrderedList")),
            RteMsg::RemoveFormat => Some(ExecCommand::new("removeFormat")),
            RteMsg::Subscript => Some(ExecCommand::new("subscript")),
            RteMsg::Superscript => Some(ExecCommand::new("superscript")),
            RteMsg::InsertTable { .. } => None,
            // code
            RteMsg::InsertCode(_) => None,
            RteMsg::CodeChanged(_) => None,
            RteMsg::InjectCode => None,

            // indent
            RteMsg::ChangeIndent(RteIndentMsg::Increase) => Some(ExecCommand::new("indent")),
            RteMsg::ChangeIndent(RteIndentMsg::Decrease) => Some(ExecCommand::new("outdent")),

            // outer
            RteMsg::TableSetColumns(..)
            | RteMsg::TableSetRows(..)
            | RteMsg::TableSetVisibility(..) => None,

            RteMsg::RequestFocus(identifier) => {
                let res = seed::document().query_selector(format!("#{}", identifier).as_str());
                if let Ok(Some(el)) = res {
                    if let Ok(el) = el.dyn_into::<web_sys::HtmlElement>() {
                        if let Err(e) = el.focus() {
                            log!(e)
                        }
                    }
                }
                None
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct StyledRteTableState {
    pub visible: bool,
    pub rows: u16,
    pub cols: u16,
}

#[derive(Debug, Clone)]
pub struct StyledRteCodeState {
    pub visible: bool,
    pub lang: StyledSelectState,
    pub code: String,
    languages: Vec<String>,
}

impl StyledRteCodeState {
    pub fn new(field_id: FieldId) -> Self {
        let mut languages: Vec<String> = crate::hi::syntax_set::load()
            .syntaxes()
            .iter()
            .map(|s| s.name.clone())
            .collect();
        languages.sort();
        Self {
            visible: false,
            lang: StyledSelectState::new(
                FieldId::Rte(RteField::CodeLang(Box::new(field_id))),
                vec![],
            ),
            code: "".to_string(),
            languages,
        }
    }

    pub fn languages(&self) -> &Vec<String> {
        &self.languages
    }

    pub fn reset(&mut self) {
        self.code.clear();
        self.lang.reset();
        self.visible = false;
    }
}

struct RteTableBodyBuilder {
    cols: u16,
    rows: u16,
}

impl RteTableBodyBuilder {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self { cols, rows }
    }
}

impl ToString for RteTableBodyBuilder {
    fn to_string(&self) -> String {
        let RteTableBodyBuilder { cols, rows } = self;
        let mut buff = "<tbody>".to_string();
        for _c in 0..(*cols) {
            buff.push_str("<tr>");
            for _r in 0..(*rows) {
                buff.push_str("<td>&nbsp;</td>")
            }
            buff.push_str("</tr>");
        }
        buff.push_str("</tbody>");
        buff
    }
}

#[derive(Debug)]
pub struct StyledRteState {
    pub value: String,
    pub field_id: FieldId,
    pub table_tooltip: StyledRteTableState,
    pub code_tooltip: StyledRteCodeState,
    range: Option<web_sys::Range>,
    identifier: uuid::Uuid,
}

impl StyledRteState {
    pub fn new(field_id: FieldId) -> Self {
        Self {
            field_id: field_id.clone(),
            value: String::new(),
            table_tooltip: StyledRteTableState {
                visible: false,
                rows: 3,
                cols: 3,
            },
            code_tooltip: StyledRteCodeState::new(field_id),
            range: None,
            identifier: uuid::Uuid::new_v4(),
        }
    }

    pub fn update(&mut self, msg: &Msg, orders: &mut impl Orders<Msg>) {
        self.code_tooltip.lang.update(msg, orders);
        let m = match msg {
            Msg::Rte(field, m) if field == &self.field_id => m,
            _ => return,
        };
        match m.to_command() {
            Some(ExecCommand { name, param }) => {
                self.store_range();
                let doc = match web_sys::window().and_then(|w| w.document()).map(|d| {
                    wasm_bindgen::JsValue::from(d).unchecked_into::<web_sys::HtmlDocument>()
                }) {
                    Some(doc) => doc,
                    _ => return,
                };
                match doc.exec_command_with_show_ui_and_value(name, false, param) {
                    Ok(_) => {}
                    Err(e) => log!(e),
                }
                if self.restore_range().is_err() {
                    return;
                }
                self.schedule_focus(orders);
            }
            _ => match m {
                // code
                RteMsg::InsertCode(b) => {
                    if *b {
                        self.store_range();
                    } else {
                        self.code_tooltip.reset();
                    }
                    self.code_tooltip.visible = *b;
                }
                RteMsg::CodeChanged(s) => {
                    self.code_tooltip.code = s.to_string();
                }
                RteMsg::InjectCode => {
                    let lang = match self
                        .code_tooltip
                        .lang
                        .values
                        .get(0)
                        .cloned()
                        .and_then(|idx| self.code_tooltip.languages.get(idx as usize))
                    {
                        Some(v) => v.to_string(),
                        _ => return,
                    };
                    let doc = seed::html_document();
                    let r = match self.range.as_ref() {
                        Some(r) => r,
                        _ => return,
                    };
                    let code = self.code_tooltip.code.to_string();
                    let view = match doc.create_element("jirs-code-view") {
                        Ok(t) => t,
                        _ => return,
                    };
                    if let Err(err) = view.set_attribute("lang", lang.as_str()) {
                        error!(err);
                    }
                    view.set_inner_html(code.as_str());
                    if let Err(e) = r.insert_node(&view) {
                        log!(e);
                    }

                    self.code_tooltip.reset();

                    self.schedule_focus(orders);
                }
                // table
                RteMsg::TableSetRows(n) => {
                    self.table_tooltip.rows = *n;
                }
                RteMsg::TableSetColumns(n) => {
                    self.table_tooltip.cols = *n;
                }
                RteMsg::TableSetVisibility(b) => {
                    if *b {
                        self.store_range();
                    }
                    self.table_tooltip.visible = *b;
                }
                RteMsg::InsertTable { rows, cols } => {
                    self.table_tooltip.visible = false;
                    self.table_tooltip.cols = 3;
                    self.table_tooltip.rows = 3;
                    if self.restore_range().is_err() {
                        return;
                    }
                    let doc = seed::html_document();
                    let r = match self.range.as_ref() {
                        Some(r) => r,
                        _ => return,
                    };
                    let table = match doc.create_element("table") {
                        Ok(t) => t,
                        _ => return,
                    };
                    table.set_inner_html(
                        RteTableBodyBuilder::new(*cols, *rows).to_string().as_str(),
                    );
                    if let Err(e) = r.insert_node(&table) {
                        log!(e);
                    }
                    self.schedule_focus(orders);
                }
                _ => log!(m),
            },
        };
        // orders.skip().send_msg(Msg::StrInputChanged(
        //     self.field_id.clone(),
        //     self.value.clone(),
        // ));
    }

    fn store_range(&mut self) {
        self.range = seed::html_document()
            .get_selection()
            .ok()
            .unwrap_or_else(|| None)
            .and_then(|s| s.get_range_at(0).ok());
    }

    fn restore_range(&mut self) -> Result<(), String> {
        let doc = seed::html_document();
        let sel = doc
            .get_selection()
            .ok()
            .unwrap_or_else(|| None)
            .ok_or_else(|| "Restoring selection failed. Unable to obtain select".to_string())?;
        let r = self
            .range
            .as_ref()
            .ok_or_else(|| "Restoring selection failed. No range was stored".to_string())?;
        sel.remove_all_ranges()
            .map_err(|_| "Restoring selection failed. Unable to remove ranges".to_string())?;
        sel.add_range(r).map_err(|_| {
            "Restoring selection failed. Unable to add current selection range".to_string()
        })?;
        Ok(())
    }

    fn schedule_focus(&self, orders: &mut impl Orders<Msg>) {
        let field_id = self.field_id.clone();
        let identifier = self.identifier;
        orders.perform_cmd(cmds::timeout(200, move || {
            Msg::Rte(field_id, RteMsg::RequestFocus(identifier))
        }));
    }
}

pub struct StyledRte<'component> {
    field_id: FieldId,
    table_tooltip: Option<&'component StyledRteTableState>,
    identifier: Option<uuid::Uuid>,
    code_tooltip: Option<&'component StyledRteCodeState>,
}

impl<'component> StyledRte<'component> {
    pub fn build(field_id: FieldId) -> StyledRteBuilder<'component> {
        StyledRteBuilder {
            field_id: field_id.clone(),
            value: String::new(),
            table_tooltip: None,
            code_tooltip: None,
            identifier: None,
        }
    }
}

impl<'component> ToNode for StyledRte<'component> {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

pub struct StyledRteBuilder<'outer> {
    field_id: FieldId,
    value: String,
    table_tooltip: Option<&'outer StyledRteTableState>,
    code_tooltip: Option<&'outer StyledRteCodeState>,
    identifier: Option<uuid::Uuid>,
}

impl<'outer> StyledRteBuilder<'outer> {
    pub fn state(mut self, state: &'outer StyledRteState) -> Self {
        self.value = state.value.clone();
        self.table_tooltip = Some(&state.table_tooltip);
        self.code_tooltip = Some(&state.code_tooltip);
        self.identifier = Some(state.identifier);
        self
    }

    pub fn build(self) -> StyledRte<'outer> {
        StyledRte {
            field_id: self.field_id,
            table_tooltip: self.table_tooltip,
            identifier: self.identifier,
            code_tooltip: self.code_tooltip,
        }
    }
}

pub fn render(values: StyledRte) -> Node<Msg> {
    /*{
        let _brush_button = styled_rte_button(
            "Brush",
            Icon::Brush,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                None as Option<Msg>
            }),
        );
        let _color_bucket_button = styled_rte_button(
            "Color bucket",
            Icon::ColorBucket,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                None as Option<Msg>
            }),
        );
        let _color_picker_button = styled_rte_button(
            "Color picker",
            Icon::ColorPicker,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                None as Option<Msg>
            }),
        );

        let _link_broken_button = styled_rte_button(
            "Link broken",
            Icon::LinkBroken,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                None as Option<Msg>
            }),
        );

        let _pin_button = styled_rte_button(
            "Pin",
            Icon::Pin,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                None as Option<Msg>
            }),
        );
        let _rotation_button = styled_rte_button(
            "Rotation",
            Icon::Rotation,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                None as Option<Msg>
            }),
        );
        let _save_button = styled_rte_button(
            "Save",
            Icon::Save,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                None as Option<Msg>
            }),
        );
        let _text_height_button = styled_rte_button(
            "Text height",
            Icon::TextHeight,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                None as Option<Msg>
            }),
        );
        let _text_width_button = styled_rte_button(
            "Text width",
            Icon::TextWidth,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                None as Option<Msg>
            }),
        );
    }*/

    // let field_id = values.field_id.clone();
    let capture_event = ev(Ev::KeyDown, |ev| {
        ev.stop_propagation();
        None as Option<Msg>
    });
    // let capture_change = ev(Ev::Input, |ev| {
    //     ev.stop_propagation();
    //     Some(Msg::StrInputChanged(field_id, "".to_string()))
    // });

    let id = values.identifier.unwrap_or_default().to_string();

    div![
        C!["styledRte"],
        attrs![At::Id => id],
        div![
            C!["bar"],
            first_row(&values),
            second_row(&values),
            // brush_button,
            // color_bucket_button,
            // color_picker_button,
            // link_broken_button,
            // pin_button,
            // save_button,
            // text_height_button,
            // text_width_button,
        ],
        div![
            C!["editorWrapper"],
            div![
                C!["editor"],
                attrs![At::ContentEditable => true],
                capture_event,
                // capture_change
            ],
        ]
    ]
}

fn first_row(values: &StyledRte) -> Node<Msg> {
    let justify = {
        let field_id = values.field_id.clone();
        let justify_all_button = styled_rte_button(
            "Justify All",
            Icon::JustifyAll,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();

                Some(Msg::Rte(field_id, RteMsg::JustifyFull))
            }),
        );
        let field_id = values.field_id.clone();
        let justify_center_button = styled_rte_button(
            "Justify Center",
            Icon::JustifyCenter,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                Some(Msg::Rte(field_id, RteMsg::JustifyCenter))
            }),
        );
        let field_id = values.field_id.clone();
        let justify_left_button = styled_rte_button(
            "Justify Left",
            Icon::JustifyLeft,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                Some(Msg::Rte(field_id, RteMsg::JustifyLeft))
            }),
        );
        let field_id = values.field_id.clone();
        let justify_right_button = styled_rte_button(
            "Justify Right",
            Icon::JustifyRight,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();

                Some(Msg::Rte(field_id, RteMsg::JustifyRight))
            }),
        );
        div![
            class!["group justify"],
            justify_all_button,
            justify_center_button,
            justify_left_button,
            justify_right_button
        ]
    };

    let system = {
        let field_id = values.field_id.clone();
        let redo_button = styled_rte_button(
            "Redo",
            Icon::Redo,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                Some(Msg::Rte(field_id, RteMsg::Redo))
            }),
        );
        let field_id = values.field_id.clone();
        let undo_button = styled_rte_button(
            "Undo",
            Icon::Undo,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                Some(Msg::Rte(field_id, RteMsg::Undo))
            }),
        );
        /*let field_id = values.field_id.clone();
        let clip_board_button = styled_rte_button(
            "Paste",
            Icon::ClipBoard,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                Some(Msg::Rte(RteMsg::Paste, field_id))
            }),
        );
        let field_id = values.field_id.clone();
        let copy_button = styled_rte_button(
            "Copy",
            Icon::Copy,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                Some(Msg::Rte(RteMsg::Copy, field_id))
            }),
        );
        let field_id = values.field_id.clone();
        let cut_button = styled_rte_button(
            "Cut",
            Icon::Cut,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                Some(Msg::Rte(RteMsg::Cut, field_id))
            }),
        );*/
        div![
            class!["group system"],
            // clip_board_button,
            // copy_button,
            // cut_button,
            undo_button,
            redo_button,
        ]
    };

    let formatting = {
        let field_id = values.field_id.clone();
        let remove_formatting = styled_rte_button(
            "Remove format",
            Icon::EraserAlt,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                Some(Msg::Rte(field_id, RteMsg::RemoveFormat))
            }),
        );
        let field_id = values.field_id.clone();
        let bold_button = styled_rte_button(
            "Bold",
            Icon::Bold,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                Some(Msg::Rte(field_id, RteMsg::Bold))
            }),
        );
        let field_id = values.field_id.clone();
        let italic_button = styled_rte_button(
            "Italic",
            Icon::Italic,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                Some(Msg::Rte(field_id, RteMsg::Italic))
            }),
        );

        let underline_button = {
            let field_id = values.field_id.clone();
            styled_rte_button(
                "Underline",
                Icon::Underline,
                mouse_ev(Ev::Click, move |ev| {
                    ev.prevent_default();
                    Some(Msg::Rte(field_id, RteMsg::Underscore))
                }),
            )
        };

        let strike_through_button = {
            let field_id = values.field_id.clone();
            styled_rte_button(
                "StrikeThrough",
                Icon::StrikeThrough,
                mouse_ev(Ev::Click, move |ev| {
                    ev.prevent_default();
                    Some(Msg::Rte(field_id, RteMsg::Strikethrough))
                }),
            )
        };

        let subscript_button = {
            let field_id = values.field_id.clone();
            styled_rte_button(
                "Subscript",
                Icon::Subscript,
                mouse_ev(Ev::Click, move |ev| {
                    ev.prevent_default();
                    Some(Msg::Rte(field_id, RteMsg::Subscript))
                }),
            )
        };

        let superscript_button = {
            let field_id = values.field_id.clone();
            styled_rte_button(
                "Superscript",
                Icon::Superscript,
                mouse_ev(Ev::Click, move |ev| {
                    ev.prevent_default();
                    Some(Msg::Rte(field_id, RteMsg::Superscript))
                }),
            )
        };

        div![
            class!["group formatting"],
            bold_button,
            italic_button,
            underline_button,
            strike_through_button,
            subscript_button,
            superscript_button,
            remove_formatting,
        ]
    };

    div![class!["row firstRow"], system, formatting, justify,]
}

fn second_row(values: &StyledRte) -> Node<Msg> {
    /*let align_group = {
        let field_id = values.field_id.clone();
        let align_center_button = styled_rte_button(
            "Align Center",
            Icon::AlignCenter,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                None as Option<Msg>
            }),
        );
        let field_id = values.field_id.clone();
        let align_left_button = styled_rte_button(
            "Align Left",
            Icon::AlignLeft,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                None as Option<Msg>
            }),
        );
        let field_id = values.field_id.clone();
        let align_right_button = styled_rte_button(
            "Align Right",
            Icon::AlignRight,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                None as Option<Msg>
            }),
        );
        div![
            class!["group align"],
            align_center_button,
            align_left_button,
            align_right_button,
        ]
    };*/

    let font_group = {
        let _field_id = values.field_id.clone();
        let _font_button = styled_rte_button(
            "Font",
            Icon::Font,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                None as Option<Msg>
            }),
        );
        let options: Vec<Node<Msg>> = HeadingSize::all()
            .into_iter()
            .map(|h| {
                let field_id = values.field_id.clone();
                let button = StyledButton::build()
                    .text(h.as_str())
                    .on_click(mouse_ev(Ev::Click, move |ev| {
                        ev.prevent_default();
                        Some(Msg::Rte(field_id, RteMsg::InsertHeading(h)))
                    }))
                    .empty()
                    .build()
                    .into_node();
                span![class!["headingOption"], button]
            })
            .collect();
        let heading_button = span![class!["headingList"], options];

        /*let _field_id = values.field_id.clone();
        let _small_cap_button = styled_rte_button(
            "Small Cap",
            Icon::SmallCap,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                None as Option<Msg>
            }),
        );
        let _field_id = values.field_id.clone();
        let _all_caps_button = styled_rte_button(
            "All caps",
            Icon::AllCaps,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                None as Option<Msg>
            }),
        );*/
        div![
            class!["group font"],
            // font_button,
            heading_button,
            // small_cap_button,
            // all_caps_button
        ]
    };

    let insert_group = {
        let table_tooltip = table_tooltip(values);

        let field_id = values.field_id.clone();
        let listing_dots = styled_rte_button(
            "Listing dots",
            Icon::ListingDots,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                Some(Msg::Rte(field_id, RteMsg::InsertUnorderedList))
            }),
        );
        let field_id = values.field_id.clone();
        let listing_number = styled_rte_button(
            "Listing number",
            Icon::ListingNumber,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                Some(Msg::Rte(field_id, RteMsg::InsertOrderedList))
            }),
        );
        /*let field_id = values.field_id.clone();
        let sub_listing_button = styled_rte_button(
            "Sub Listing",
            Icon::SubListing,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                None as Option<Msg>
            }),
        );*/

        let mut table_button = {
            let field_id = values.field_id.clone();
            styled_rte_button(
                "Table",
                Icon::Table,
                mouse_ev(Ev::Click, move |ev| {
                    ev.prevent_default();
                    Some(Msg::Rte(field_id, RteMsg::TableSetVisibility(true)))
                }),
            )
        };
        table_button.add_child(table_tooltip);

        let paragraph_button = {
            let field_id = values.field_id.clone();
            styled_rte_button(
                "Paragraph",
                Icon::Paragraph,
                mouse_ev(Ev::Click, move |ev| {
                    ev.prevent_default();
                    Some(Msg::Rte(field_id, RteMsg::InsertParagraph))
                }),
            )
        };
        let mut code_alt_button = {
            let field_id = values.field_id.clone();
            styled_rte_button(
                "Insert code",
                Icon::CodeAlt,
                mouse_ev(Ev::Click, move |ev| {
                    ev.prevent_default();
                    Some(Msg::Rte(field_id, RteMsg::InsertCode(true)))
                }),
            )
        };
        code_alt_button.add_child(code_tooltip(values));

        div![
            class!["group insert"],
            paragraph_button,
            table_button,
            code_alt_button,
            listing_dots,
            listing_number,
            // sub_listing_button,
        ]
    };

    let indent_outdent = {
        let field_id = values.field_id.clone();
        let indent_button = styled_rte_button(
            "Indent",
            Icon::Indent,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                Some(Msg::Rte(
                    field_id,
                    RteMsg::ChangeIndent(RteIndentMsg::Increase),
                ))
            }),
        );
        let field_id = values.field_id.clone();
        let outdent_button = styled_rte_button(
            "Outdent",
            Icon::Outdent,
            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                Some(Msg::Rte(
                    field_id,
                    RteMsg::ChangeIndent(RteIndentMsg::Decrease),
                ))
            }),
        );
        div![class!["group indentOutdent"], indent_button, outdent_button]
    };

    div![
        class!["row secondRow"],
        font_group,
        // align_group,
        insert_group,
        indent_outdent
    ]
}

fn table_tooltip(values: &StyledRte) -> Node<Msg> {
    let (visible, rows, cols) = values
        .table_tooltip
        .map(
            |StyledRteTableState {
                 visible,
                 rows,
                 cols,
             }| (*visible, *rows, *cols),
        )
        .unwrap_or_default();

    let on_rows_change = {
        let field_id = values.field_id.clone();
        input_ev(Ev::Change, move |v| {
            v.parse::<u16>()
                .ok()
                .map(|n| Msg::Rte(field_id, RteMsg::TableSetRows(n)))
        })
    };

    let on_cols_change = {
        let field_id = values.field_id.clone();
        input_ev(Ev::Change, move |v| {
            v.parse::<u16>()
                .ok()
                .map(|n| Msg::Rte(field_id, RteMsg::TableSetColumns(n)))
        })
    };

    let close_table_tooltip = {
        let field_id = values.field_id.clone();
        StyledButton::build()
            .empty()
            .icon(Icon::Close)
            .on_click(mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                Some(Msg::Rte(field_id, RteMsg::TableSetVisibility(false)))
            }))
            .build()
            .into_node()
    };

    let on_submit = {
        let field_id = values.field_id.clone();
        mouse_ev(Ev::Click, move |ev| {
            ev.prevent_default();
            Some(Msg::Rte(field_id, RteMsg::InsertTable { rows, cols }))
        })
    };

    StyledTooltip::build()
        .table_tooltip()
        .visible(visible)
        .add_child(h2![span!["Add table"], close_table_tooltip])
        .add_child(div![class!["inputs"], span!["Rows"], seed::input![
                attrs![At::Type => "range"; At::Step => "1"; At::Min => "1"; At::Max => "10"; At::Value => rows],
                on_rows_change
            ]])
        .add_child(div![
            class!["inputs"],
            span!["Columns"],
            seed::input![
                attrs![At::Type => "range"; At::Step => "1"; At::Min => "1"; At::Max => "10"; At::Value => cols],
                on_cols_change
            ]
        ])
        .add_child({
            let body: Vec<Node<Msg>> = (0..rows)
                .map(|_row| {
                    let tds: Vec<Node<Msg>> = (0..cols)
                        .map(|_col| td![" "])
                        .collect();
                    tr![tds]
                })
                .collect();
            seed::div![
                class!["tablePreview"],
                seed::table![tbody![body]],
                input![attrs![At::Type => "button"; At::Value => "Insert"], on_submit],
            ]
        })
        .build()
        .into_node()
}

fn styled_rte_button(title: &str, icon: Icon, handler: EventHandler<Msg>) -> Node<Msg> {
    let button = StyledButton::build()
        .icon(StyledIcon::build(icon).build())
        .on_click(handler)
        .empty()
        .build()
        .into_node();
    span![
        class!["styledRteButton"],
        attrs![At::Title => title],
        button
    ]
}

fn code_tooltip(values: &StyledRte) -> Node<Msg> {
    let (visible, lang) = values
        .code_tooltip
        .as_ref()
        .map(|StyledRteCodeState { visible, lang, .. }| (*visible, Some(lang)))
        .unwrap_or_default();

    let languages = values
        .code_tooltip
        .map(|ct| ct.languages().as_slice())
        .unwrap_or_default();

    let options: Vec<(String, u32)> = languages
        .iter()
        .enumerate()
        .map(|(idx, label)| (label.to_string(), idx as u32))
        .collect();

    let select_lang_node = StyledSelect::build()
        .try_state(lang)
        .selected(
            lang.and_then(|l| l.values.get(0))
                .and_then(|n| options.get(*n as usize))
                .map(|v| vec![v.to_child()])
                .unwrap_or_default(),
        )
        .options(options.iter().map(|opt| opt.to_child()).collect())
        .normal()
        .build(FieldId::Rte(RteField::CodeLang(Box::new(
            values.field_id.clone(),
        ))))
        .into_node();

    let close_tooltip = {
        let field_id = values.field_id.clone();
        StyledButton::build()
            .empty()
            .icon(Icon::Close)
            .on_click(mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                Some(Msg::Rte(field_id, RteMsg::InsertCode(false)))
            }))
            .build()
            .into_node()
    };

    let input = {
        let field_id = values.field_id.clone();
        let on_change = ev(Ev::Change, move |ev| {
            ev.stop_propagation();
            let target = ev.target().unwrap();
            let textarea = seed::to_textarea(&target);
            let code = textarea.value();
            Msg::Rte(field_id, RteMsg::CodeChanged(code))
        });
        seed::textarea![on_change]
    };

    let actions = {
        let field_id = values.field_id.clone();
        let on_insert = ev(Ev::Click, move |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            ev.target().unwrap();
            Msg::Rte(field_id, RteMsg::InjectCode)
        });
        let insert = StyledButton::build()
            .on_click(on_insert)
            .text("Insert")
            .build()
            .into_node();
        div![insert]
    };

    StyledTooltip::build()
        .code_tooltip()
        .visible(visible)
        .add_child(h2!["Insert Code", close_tooltip])
        .add_child(select_lang_node)
        .add_child(input)
        .add_child(actions)
        .build()
        .into_node()
}
