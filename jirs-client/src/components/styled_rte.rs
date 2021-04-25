use seed::prelude::*;
use seed::*;

use crate::components::styled_button::{ButtonVariant, StyledButton};
use crate::components::styled_icon::{Icon, StyledIcon};
use crate::components::styled_select::{SelectVariant, StyledSelect, StyledSelectState};
use crate::components::styled_select_child::StyledSelectOption;
use crate::components::styled_tooltip::{StyledTooltip, TooltipVariant};
use crate::{ButtonId, FieldId, Msg, RteField};

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
    fn all<'l>() -> &'l [Self; 7] {
        use HeadingSize::*;

        &[Normal, H1, H2, H3, H4, H5, H6]
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
                            error!(e)
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
        let mut languages: Vec<String> = vec![]; /*crate::hi::syntax_set::load()
                                                 .syntaxes()
                                                 .iter()
                                                 .map(|s| s.name.clone())
                                                 .collect();*/
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
    pub identifier: uuid::Uuid,
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
                if let Err(e) = doc.exec_command_with_show_ui_and_value(name, false, param) {
                    error!(e)
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
                        .and_then(|idx| self.code_tooltip.languages.get(*idx as usize))
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
                        error!(e);
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
                        error!(e);
                    }
                    self.schedule_focus(orders);
                }
                _ => log::error!("unknown rte command {:?}", m),
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
            .unwrap_or(None)
            .and_then(|s| s.get_range_at(0).ok());
    }

    fn restore_range(&mut self) -> Result<(), String> {
        let doc = seed::html_document();
        let sel = doc
            .get_selection()
            .ok()
            .unwrap_or(None)
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
    pub field_id: FieldId,
    pub table_tooltip: Option<&'component StyledRteTableState>,
    pub identifier: Option<uuid::Uuid>,
    pub code_tooltip: Option<&'component StyledRteCodeState>,
}

impl<'component> Default for StyledRte<'component> {
    fn default() -> Self {
        Self {
            field_id: FieldId::NoField,
            table_tooltip: None,
            identifier: None,
            code_tooltip: None,
        }
    }
}

impl<'outer> StyledRte<'outer> {
    pub fn render(self) -> Node<Msg> {
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
        // let capture_change = ev(Ev::Input, |ev| {
        //     ev.stop_propagation();
        //     Some(Msg::StrInputChanged(field_id, "".to_string()))
        // });

        let id = self.identifier.unwrap_or_default().to_string();

        let click_handler = {
            let field_id = self.field_id.clone();
            let (rows, cols) = self
                .table_tooltip
                .as_ref()
                .map(|t| (t.rows, t.cols))
                .unwrap_or_default();

            mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                ev.stop_propagation();

                let target = ev
                    .current_target()
                    .map(|el| seed::to_html_el(&el).id())
                    .unwrap_or_default();

                let rte_msg = match target.as_str() {
                    "justifyAll" => RteMsg::JustifyFull,
                    "justifyCenter" => RteMsg::JustifyCenter,
                    "justifyLeft" => RteMsg::JustifyLeft,
                    "justifyRight" => RteMsg::JustifyRight,
                    "undo" => RteMsg::Undo,
                    "redo" => RteMsg::Redo,

                    "removeFormat" => RteMsg::RemoveFormat,
                    "bold" => RteMsg::Bold,
                    "italic " => RteMsg::Italic,
                    "underscore" => RteMsg::Underscore,
                    "strikethrough" => RteMsg::Strikethrough,
                    "subscript" => RteMsg::Subscript,
                    "superscript" => RteMsg::Superscript,

                    // "font" => RteMsg::, // Some(RteMsg::Font),
                    "listingDots" => RteMsg::InsertUnorderedList,
                    "listingNumber" => RteMsg::InsertOrderedList,
                    "table" => RteMsg::TableSetVisibility(true),
                    "paragraph" => RteMsg::InsertParagraph,
                    "codeAlt" => RteMsg::InsertCode(true),
                    "indent" => RteMsg::ChangeIndent(RteIndentMsg::Increase),
                    "outdent" => RteMsg::ChangeIndent(RteIndentMsg::Decrease),

                    "closeRteTableTooltip" => RteMsg::TableSetVisibility(false),
                    "rteInsertCode" => RteMsg::InsertCode(false),
                    "rteInjectCode" => RteMsg::InjectCode,
                    "rteInsertTable" => RteMsg::InsertTable { rows, cols },

                    _ => {
                        let target = ev.target().unwrap();
                        let h = seed::to_html_el(&target);
                        error!("unknown rte command for element", h);
                        unreachable!();
                    }
                };
                Msg::Rte(field_id, rte_msg)
            })
        };

        let change_handler = {
            let field_id = self.field_id.clone();
            ev(Ev::Change, move |event| {
                event
                    .target()
                    .as_ref()
                    .ok_or("Can't get event target reference")
                    .and_then(util::get_value)
                    .ok()
                    .and_then(|s| s.parse::<u16>().ok())
                    .map(|n| Msg::Rte(field_id, RteMsg::TableSetRows(n)))
            })
        };

        div![
            C!["styledRte"],
            attrs![At::Id => id],
            div![
                C!["bar"],
                Self::first_row(click_handler.clone()),
                self.second_row(click_handler, change_handler),
                /* brush_button,
                 * color_bucket_button,
                 * color_picker_button,
                 * link_broken_button,
                 * pin_button,
                 * save_button,
                 * text_height_button,
                 * text_width_button, */
            ],
            div![
                C!["editorWrapper"],
                div![
                    C!["editor", self.field_id.to_str()],
                    attrs![At::ContentEditable => true],
                    // capture_change
                ],
            ]
        ]
    }

    fn first_row(click_handler: EventHandler<Msg>) -> Node<Msg> {
        let justify = {
            let justify_all_button = Self::styled_rte_button(
                "Justify All",
                ButtonId::JustifyAll,
                Icon::JustifyAll,
                click_handler.clone(),
            );
            let justify_center_button = Self::styled_rte_button(
                "Justify Center",
                ButtonId::JustifyCenter,
                Icon::JustifyCenter,
                click_handler.clone(),
            );
            let justify_left_button = Self::styled_rte_button(
                "Justify Left",
                ButtonId::JustifyLeft,
                Icon::JustifyLeft,
                click_handler.clone(),
            );
            let justify_right_button = Self::styled_rte_button(
                "Justify Right",
                ButtonId::JustifyRight,
                Icon::JustifyRight,
                click_handler.clone(),
            );
            div![
                C!["group justify"],
                justify_all_button,
                justify_center_button,
                justify_left_button,
                justify_right_button
            ]
        };

        let system = {
            let redo_button =
                Self::styled_rte_button("Redo", ButtonId::Redo, Icon::Redo, click_handler.clone());
            let undo_button =
                Self::styled_rte_button("Undo", ButtonId::Undo, Icon::Undo, click_handler.clone());
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
                C!["group system"],
                // clip_board_button,
                // copy_button,
                // cut_button,
                undo_button,
                redo_button,
            ]
        };

        let formatting = {
            let remove_formatting = Self::styled_rte_button(
                "Remove format",
                ButtonId::RemoveFormat,
                Icon::EraserAlt,
                click_handler.clone(),
            );
            let bold_button =
                Self::styled_rte_button("Bold", ButtonId::Bold, Icon::Bold, click_handler.clone());
            let italic_button = Self::styled_rte_button(
                "Italic",
                ButtonId::Italic,
                Icon::Italic,
                click_handler.clone(),
            );

            let underline_button = Self::styled_rte_button(
                "Underline",
                ButtonId::Underscore,
                Icon::Underline,
                click_handler.clone(),
            );

            let strike_through_button = Self::styled_rte_button(
                "StrikeThrough",
                ButtonId::Strikethrough,
                Icon::StrikeThrough,
                click_handler.clone(),
            );

            let subscript_button = Self::styled_rte_button(
                "Subscript",
                ButtonId::Subscript,
                Icon::Subscript,
                click_handler.clone(),
            );

            let superscript_button = Self::styled_rte_button(
                "Superscript",
                ButtonId::Superscript,
                Icon::Superscript,
                click_handler,
            );

            div![
                C!["group formatting"],
                bold_button,
                italic_button,
                underline_button,
                strike_through_button,
                subscript_button,
                superscript_button,
                remove_formatting,
            ]
        };

        div![C!["row firstRow"], system, formatting, justify]
    }

    fn second_row(
        &self,
        click_handler: EventHandler<Msg>,
        change_handler: EventHandler<Msg>,
    ) -> Node<Msg> {
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
                C!["group align"],
                align_center_button,
                align_left_button,
                align_right_button,
            ]
        };*/

        let font_group = {
            let _font_button =
                Self::styled_rte_button("Font", ButtonId::Font, Icon::Font, click_handler.clone());
            let options: Vec<Node<Msg>> = HeadingSize::all()
                .iter()
                .map(|h| {
                    let field_id = self.field_id.clone();
                    let button = StyledButton {
                        text: Some(h.as_str()),
                        on_click: Some(mouse_ev(Ev::Click, move |ev| {
                            ev.prevent_default();
                            Some(Msg::Rte(field_id, RteMsg::InsertHeading(*h)))
                        })),
                        variant: ButtonVariant::Empty,
                        ..Default::default()
                    }
                    .render();
                    span![C!["headingOption"], button]
                })
                .collect();
            let heading_button = span![C!["headingList"], options];

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
                C!["group font"],
                // font_button,
                heading_button,
                /* small_cap_button,
                 * all_caps_button */
            ]
        };

        let insert_group = {
            let table_tooltip = self.table_tooltip(click_handler.clone(), change_handler);

            let listing_dots = Self::styled_rte_button(
                "Listing dots",
                ButtonId::ListingDots,
                Icon::ListingDots,
                click_handler.clone(),
            );
            let listing_number = Self::styled_rte_button(
                "Listing number",
                ButtonId::ListingNumber,
                Icon::ListingNumber,
                click_handler.clone(),
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

            let mut table_button = Self::styled_rte_button(
                "Table",
                ButtonId::Table,
                Icon::Table,
                click_handler.clone(),
            );
            table_button.add_child(table_tooltip);

            let paragraph_button = Self::styled_rte_button(
                "Paragraph",
                ButtonId::Paragraph,
                Icon::Paragraph,
                click_handler.clone(),
            );
            let mut code_alt_button = Self::styled_rte_button(
                "Insert code",
                ButtonId::CodeAlt,
                Icon::CodeAlt,
                click_handler.clone(),
            );
            code_alt_button.add_child(self.code_tooltip(click_handler.clone()));

            div![
                C!["group insert"],
                paragraph_button,
                table_button,
                code_alt_button,
                listing_dots,
                listing_number,
                // sub_listing_button,
            ]
        };

        let indent_outdent = {
            let indent_button = Self::styled_rte_button(
                "Indent",
                ButtonId::Indent,
                Icon::Indent,
                click_handler.clone(),
            );
            let outdent_button =
                Self::styled_rte_button("Outdent", ButtonId::Outdent, Icon::Outdent, click_handler);
            div![C!["group indentOutdent"], indent_button, outdent_button]
        };

        div![
            C!["row secondRow"],
            font_group,
            // align_group,
            insert_group,
            indent_outdent
        ]
    }

    fn table_tooltip(
        &self,
        click_handler: EventHandler<Msg>,
        _change_handler: EventHandler<Msg>,
    ) -> Node<Msg> {
        let (visible, rows, cols) = self
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
            let field_id = self.field_id.clone();
            input_ev(Ev::Change, move |v| {
                v.parse::<u16>()
                    .ok()
                    .map(|n| Msg::Rte(field_id, RteMsg::TableSetRows(n)))
            })
        };

        let on_cols_change = {
            let field_id = self.field_id.clone();
            input_ev(Ev::Change, move |v| {
                v.parse::<u16>()
                    .ok()
                    .map(|n| Msg::Rte(field_id, RteMsg::TableSetColumns(n)))
            })
        };

        let close_table_tooltip = StyledButton {
            button_id: Some(ButtonId::CloseRteTableTooltip),
            variant: ButtonVariant::Empty,
            icon: Some(StyledIcon::from(Icon::Close).render()),
            on_click: Some(click_handler.clone()),
            ..Default::default()
        }
        .render();

        let on_submit = click_handler;

        StyledTooltip {
            variant: TooltipVariant::TableBuilder,
            visible,
            children: vec![
                h2![span!["Add table"], close_table_tooltip],
                div![C!["inputs"], span!["Rows"], seed::input![
                attrs![At::Type => "range"; At::Step => "1"; At::Min => "1"; At::Max => "10"; At::Value => rows],
                on_rows_change
            ]],
                div![
            C!["inputs"],
            span!["Columns"],
            seed::input![
                attrs![At::Type => "range"; At::Step => "1"; At::Min => "1"; At::Max => "10"; At::Value => cols],
                on_cols_change
            ]
        ],
                {
                    let body: Vec<Node<Msg>> = (0..rows)
                        .map(|_row| {
                            let tds: Vec<Node<Msg>> = (0..cols)
                                .map(|_col| td![" "])
                                .collect();
                            tr![tds]
                        })
                        .collect();
                    seed::div![
                C!["tablePreview"],
                seed::table![tbody![body]],
                input![attrs![At::Type => "button"; At::Id => "rteInsertTable"; At::Value => "Insert"], on_submit],
            ]
                }
            ],
            class_list: "",
        }.render()
    }

    fn code_tooltip(&self, click_handler: EventHandler<Msg>) -> Node<Msg> {
        let (visible, lang) = self
            .code_tooltip
            .as_ref()
            .map(|StyledRteCodeState { visible, lang, .. }| (*visible, Some(lang)))
            .unwrap_or_default();

        let languages = self
            .code_tooltip
            .map(|ct| ct.languages().as_slice())
            .unwrap_or_default();

        let options: Vec<(String, u32)> = languages
            .iter()
            .enumerate()
            .map(|(idx, label)| (label.to_string(), idx as u32))
            .collect();

        let select_lang_node = StyledSelect {
            id: FieldId::Rte(RteField::CodeLang(Box::new(self.field_id.clone()))),
            variant: SelectVariant::Normal,
            dropdown_width: None,
            name: "",
            valid: true,
            options: Some(options.iter().map(|opt| StyledSelectOption {
                text: Some(opt.0.as_str()),
                value: opt.1,
                ..Default::default()
            })),
            selected: lang
                .and_then(|l| l.values.get(0))
                .and_then(|n| options.get(*n as usize))
                .map(|v| {
                    vec![StyledSelectOption {
                        text: Some(v.0.as_str()),
                        value: v.1,
                        ..Default::default()
                    }]
                })
                .unwrap_or_default(),
            text_filter: lang.map(|l| l.text_filter.as_str()).unwrap_or_default(),
            opened: lang.map(|l| l.opened).unwrap_or_default(),
            clearable: true,
            ..Default::default()
        }
        .render();

        let close_tooltip = StyledButton {
            button_id: Some(ButtonId::RteInsertCode),
            icon: Some(StyledIcon::from(Icon::Close).render()),
            variant: ButtonVariant::Empty,
            on_click: Some(click_handler.clone()),
            ..Default::default()
        }
        .render();

        let input = {
            let field_id = self.field_id.clone();
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
            let insert = StyledButton {
                button_id: Some(ButtonId::RteInjectCode),
                on_click: Some(click_handler),
                text: Some("Insert"),
                ..Default::default()
            }
            .render();
            div![insert]
        };

        StyledTooltip {
            variant: TooltipVariant::CodeBuilder,
            children: vec![
                h2!["Insert Code", close_tooltip],
                select_lang_node,
                input,
                actions,
            ],
            visible,
            class_list: "",
        }
        .render()
    }

    fn styled_rte_button(
        title: &str,
        button_id: ButtonId,
        icon: Icon,
        handler: EventHandler<Msg>,
    ) -> Node<Msg> {
        let button = StyledButton {
            button_id: Some(button_id),
            icon: Some(StyledIcon::from(icon).render()),
            on_click: Some(handler),
            variant: ButtonVariant::Empty,
            ..Default::default()
        }
        .render();
        span![C!["styledRteButton"], attrs![At::Title => title], button]
    }
}
