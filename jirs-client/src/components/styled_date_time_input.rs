use std::ops::RangeInclusive;

use chrono::prelude::*;
use chrono::Duration;
use seed::prelude::*;
use seed::*;

use crate::components::styled_button::{ButtonVariant, StyledButton};
use crate::components::styled_icon::Icon;
use crate::components::styled_tooltip::{StyledTooltip, TooltipVariant};
use crate::shared::ToNode;
use crate::{FieldId, Msg};

#[derive(Debug)]
pub enum StyledDateTimeChanged {
    MonthChanged(Option<NaiveDateTime>),
    DayChanged(Option<NaiveDateTime>),
    PopupVisibilityChanged(bool),
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct StyledDateTimeInputState {
    field_id: FieldId,
    pub timestamp: Option<chrono::NaiveDateTime>,
    pub popup_visible: bool,
}

impl StyledDateTimeInputState {
    pub fn new(field_id: FieldId, timestamp: Option<NaiveDateTime>) -> Self {
        Self {
            field_id,
            timestamp,
            popup_visible: false,
        }
    }

    pub fn reset(&mut self) {
        self.timestamp = None;
        self.popup_visible = false;
    }

    pub fn update(&mut self, msg: &Msg, _orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::StyledDateTimeInputChanged(
                field_id,
                StyledDateTimeChanged::MonthChanged(new_date),
            ) if field_id == &self.field_id => {
                self.timestamp = *new_date;
            }
            Msg::StyledDateTimeInputChanged(
                field_id,
                StyledDateTimeChanged::DayChanged(new_date),
            ) if field_id == &self.field_id => {
                self.timestamp = *new_date;
                self.popup_visible = false;
            }
            Msg::StyledDateTimeInputChanged(
                field_id,
                StyledDateTimeChanged::PopupVisibilityChanged(b),
            ) if field_id == &self.field_id => {
                if *b {
                    self.popup_visible = true;
                } else {
                    self.reset();
                }
            }
            _ => {}
        }
    }
}

pub struct StyledDateTimeInput {
    pub field_id: FieldId,
    pub timestamp: Option<chrono::NaiveDateTime>,
    pub popup_visible: bool,
}

impl StyledDateTimeInput {
    pub fn render(self) -> Node<Msg> {
        let timestamp = self
            .timestamp
            .unwrap_or_else(|| chrono::Utc::now().naive_utc());
        let start = timestamp.with_day0(0).unwrap();
        let end = (start + Duration::days(32)).with_day0(0).unwrap();

        let calendar_start = StyledDateTimeInput::calendar_start(start.clone());
        let calendar_end = StyledDateTimeInput::calendar_end(end.clone());
        let current_month_range = start..=end;
        let mut current = calendar_start;
        let mut weeks = vec![];
        let range = calendar_start..=calendar_end;
        let mut current_week = vec![];
        loop {
            if !range.contains(&current) {
                break;
            }

            if current.weekday() == Weekday::Mon && !current_week.is_empty() {
                weeks.push(div![C!["week"], current_week]);
                current_week = vec![];
            }

            current_week.push(
                DayCell {
                    field_id: &self.field_id,
                    timestamp: &timestamp,
                    current: &current,
                    current_month_range: &current_month_range,
                }
                .render(),
            );

            current += Duration::days(1);
        }
        if !current_week.is_empty() {
            weeks.push(div![C!["week"], current_week]);
        }

        let left_action = {
            let field_id = self.field_id.clone();
            let current = timestamp;
            let on_click_left = mouse_ev(Ev::Click, move |ev| {
                ev.stop_propagation();
                ev.prevent_default();
                let last_day_of_prev_month = current.with_day0(0).unwrap() - Duration::days(1);

                let date = last_day_of_prev_month
                    .with_day0(timestamp.day0())
                    .unwrap_or(last_day_of_prev_month);
                Msg::StyledDateTimeInputChanged(
                    field_id,
                    StyledDateTimeChanged::MonthChanged(Some(date)),
                )
            });
            StyledButton {
                on_click: Some(on_click_left),
                icon: Some(Icon::DoubleLeft.into_node()),
                variant: ButtonVariant::Empty,
                ..Default::default()
            }
            .into_node()
        };
        let right_action = {
            let field_id = self.field_id.clone();
            let current = timestamp;
            let on_click_right = mouse_ev(Ev::Click, move |ev| {
                ev.stop_propagation();
                ev.prevent_default();
                let first_day_of_next_month = (current + Duration::days(32)).with_day0(0).unwrap();
                let last_day_of_next_month = (first_day_of_next_month + Duration::days(32))
                    .with_day0(0)
                    .unwrap()
                    - Duration::days(1);
                let date = first_day_of_next_month
                    .with_day0(timestamp.day0())
                    .unwrap_or(last_day_of_next_month);
                Msg::StyledDateTimeInputChanged(
                    field_id,
                    StyledDateTimeChanged::MonthChanged(Some(date)),
                )
            });
            StyledButton {
                on_click: Some(on_click_right),
                icon: Some(Icon::DoubleRight.into_node()),
                variant: ButtonVariant::Empty,
                ..Default::default()
            }
            .render()
        };

        let tooltip = StyledTooltip {
            visible: self.popup_visible,
            class_list: "",
            children: vec![
                h2![
                    left_action,
                    span![current.format("%B %Y").to_string()],
                    right_action
                ],
                div![
                    C!["calendar"],
                    div![
                        C!["weekHeader week"],
                        div![C!["day"], format!("{}", Weekday::Mon).as_str()],
                        div![C!["day"], format!("{}", Weekday::Tue).as_str()],
                        div![C!["day"], format!("{}", Weekday::Wed).as_str()],
                        div![C!["day"], format!("{}", Weekday::Thu).as_str()],
                        div![C!["day"], format!("{}", Weekday::Fri).as_str()],
                        div![C!["day"], format!("{}", Weekday::Sat).as_str()],
                        div![C!["day"], format!("{}", Weekday::Sun).as_str()],
                    ],
                    weeks
                ],
            ],
            variant: TooltipVariant::DateTimeBuilder,
        }
        .render();

        let input = {
            let field_id = self.field_id.clone();
            let visible = self.popup_visible;
            let on_focus = ev(Ev::Click, move |ev| {
                ev.prevent_default();
                ev.stop_propagation();
                Msg::StyledDateTimeInputChanged(
                    field_id,
                    StyledDateTimeChanged::PopupVisibilityChanged(!visible),
                )
            });
            let text = self
                .timestamp
                .unwrap_or_else(|| Utc::now().naive_utc())
                .date()
                .format("%d/%m/%Y")
                .to_string();
            StyledButton {
                on_click: Some(on_focus),
                text: Some(text.as_str()),
                variant: ButtonVariant::Empty,
                ..Default::default()
            }
            .render()
        };

        div![
            C!["styledDateTimeInput", format!("{}", self.field_id)],
            input,
            tooltip,
        ]
    }

    #[inline(always)]
    fn calendar_start(start: NaiveDateTime) -> NaiveDateTime {
        match start.weekday() {
            Weekday::Mon => start,
            Weekday::Tue => start - Duration::days(1),
            Weekday::Wed => start - Duration::days(2),
            Weekday::Thu => start - Duration::days(3),
            Weekday::Fri => start - Duration::days(4),
            Weekday::Sat => start - Duration::days(5),
            Weekday::Sun => start - Duration::days(6),
        }
    }

    #[inline(always)]
    fn calendar_end(end: NaiveDateTime) -> NaiveDateTime {
        match end.weekday() {
            Weekday::Mon => end + Duration::days(6),
            Weekday::Tue => end + Duration::days(5),
            Weekday::Wed => end + Duration::days(4),
            Weekday::Thu => end + Duration::days(3),
            Weekday::Fri => end + Duration::days(2),
            Weekday::Sat => end + Duration::days(1),
            Weekday::Sun => end,
        }
    }
}

impl ToNode for StyledDateTimeInput {
    fn into_node(self) -> Node<Msg> {
        self.render()
    }
}

pub struct DayCell<'l> {
    field_id: &'l FieldId,
    timestamp: &'l NaiveDateTime,
    current: &'l NaiveDateTime,
    current_month_range: &'l RangeInclusive<NaiveDateTime>,
}

impl<'l> DayCell<'l> {
    pub fn render(self) -> Node<Msg> {
        let on_click = {
            let field_id = self.field_id.clone();
            let date = *self.current;
            ev(Ev::Click, move |ev| {
                ev.stop_propagation();
                ev.prevent_default();
                Msg::StyledDateTimeInputChanged(
                    field_id,
                    StyledDateTimeChanged::DayChanged(Some(date)),
                )
            })
        };
        div![
            C![
                "day",
                format!("{}", self.current.weekday()),
                IF![self.is_selected() => "selected"],
                if self.current_month_range.contains(&self.current) {
                    "inCurrentMonth"
                } else {
                    "outCurrentMonth"
                },
            ],
            format!("{}", self.current.day()).as_str(),
            on_click,
        ]
    }

    fn is_selected(&self) -> bool {
        *self.timestamp == *self.current
    }
}
