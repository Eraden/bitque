use std::ops::RangeInclusive;

use chrono::Duration;

use {
    chrono::prelude::*,
    seed::{prelude::*, *},
};

use crate::shared::styled_button::StyledButton;
use crate::shared::styled_icon::Icon;
use crate::shared::styled_tooltip::StyledTooltip;
use crate::{shared::ToNode, FieldId, Msg};

#[derive(Debug)]
pub enum DateTimeMsg {
    MonthChanged(Option<NaiveDateTime>),
    DayChanged(Option<NaiveDateTime>),
    PopupVisibilityChanged(bool),
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct StyledDateTimeInputState {
    field_id: FieldId,
    timestamp: Option<chrono::NaiveDateTime>,
    popup_visible: bool,
}

impl StyledDateTimeInputState {
    pub fn new(field_id: FieldId, timestamp: Option<NaiveDateTime>) -> Self {
        Self {
            field_id,
            timestamp,
            popup_visible: false,
        }
    }

    pub fn update(&mut self, msg: &Msg, _orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::StyledDateTimeInputChanged(field_id, DateTimeMsg::MonthChanged(new_date))
                if field_id == &self.field_id =>
            {
                self.timestamp = *new_date;
            }
            Msg::StyledDateTimeInputChanged(field_id, DateTimeMsg::DayChanged(new_date))
                if field_id == &self.field_id =>
            {
                self.timestamp = *new_date;
            }
            Msg::StyledDateTimeInputChanged(field_id, DateTimeMsg::PopupVisibilityChanged(b))
                if field_id == &self.field_id =>
            {
                self.popup_visible = *b;
            }
            _ => {}
        }
    }
}

pub struct StyledDateTimeInput {
    field_id: FieldId,
    timestamp: Option<chrono::NaiveDateTime>,
    popup_visible: bool,
}

impl StyledDateTimeInput {
    pub fn build() -> StyledDateTimeInputBuilder {
        StyledDateTimeInputBuilder {
            timestamp: None,
            popup_visible: false,
        }
    }
}

impl ToNode for StyledDateTimeInput {
    fn into_node(self) -> Node<Msg> {
        render(self)
    }
}

pub struct StyledDateTimeInputBuilder {
    timestamp: Option<chrono::NaiveDateTime>,
    popup_visible: bool,
}

impl StyledDateTimeInputBuilder {
    pub fn state(mut self, state: &StyledDateTimeInputState) -> Self {
        self.timestamp = state.timestamp;
        self.popup_visible = state.popup_visible;
        self
    }

    pub fn build(self, field_id: FieldId) -> StyledDateTimeInput {
        StyledDateTimeInput {
            field_id,
            timestamp: self.timestamp,
            popup_visible: self.popup_visible,
        }
    }
}

fn render(values: StyledDateTimeInput) -> Node<Msg> {
    let current = values
        .timestamp
        .unwrap_or_else(|| chrono::Utc::now().naive_utc());
    let start = current.with_day0(0).unwrap().date();
    let end = (start + Duration::days(32)).with_day0(0).unwrap();

    let calendar_start = match start.weekday() {
        Weekday::Mon => start,
        Weekday::Tue => start - Duration::days(1),
        Weekday::Wed => start - Duration::days(2),
        Weekday::Thu => start - Duration::days(3),
        Weekday::Fri => start - Duration::days(4),
        Weekday::Sat => start - Duration::days(5),
        Weekday::Sun => start - Duration::days(6),
    };
    let calendar_end = match end.weekday() {
        Weekday::Mon => end + Duration::days(6),
        Weekday::Tue => end + Duration::days(5),
        Weekday::Wed => end + Duration::days(4),
        Weekday::Thu => end + Duration::days(3),
        Weekday::Fri => end + Duration::days(2),
        Weekday::Sat => end + Duration::days(1),
        Weekday::Sun => end,
    };
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

        current_week.push(day_cell(&current, &current_month_range));

        current += Duration::days(1);
    }
    if !current_week.is_empty() {
        weeks.push(div![C!["week"], current_week]);
    }

    let close_tooltip = {
        let field_id = values.field_id.clone();
        StyledButton::build()
            .empty()
            .icon(Icon::Close)
            .on_click(mouse_ev(Ev::Click, move |ev| {
                ev.prevent_default();
                Some(Msg::StyledDateTimeInputChanged(
                    field_id,
                    DateTimeMsg::PopupVisibilityChanged(false),
                ))
            }))
            .build()
            .into_node()
    };

    let tooltip = StyledTooltip::build()
        .visible(values.popup_visible)
        .add_class("dateTimeTooltip")
        .add_child(h2![span!["Add table"], close_tooltip])
        .add_child(div![
            C!["actions"],
            button![C!["prev"], "Prev"],
            button![C!["next"], "Next"],
        ])
        .add_child(div![
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
        ])
        .build()
        .into_node();

    let input = {
        let field_id = values.field_id.clone();
        let on_focus = ev(Ev::Click, move |ev| {
            ev.prevent_default();
            ev.stop_propagation();
            Msg::StyledDateTimeInputChanged(field_id, DateTimeMsg::PopupVisibilityChanged(true))
        });
        let text = values
            .timestamp
            .map(|d| format!("{}", d))
            .unwrap_or_default();
        StyledButton::build()
            .add_class("")
            .on_click(on_focus)
            .text(text)
            .active(true)
            .empty()
            .build()
            .into_node()
    };

    div![
        C!["styledDateTimeInput"],
        attrs![At::Class => format!("{}", values.field_id).as_str()],
        input,
        tooltip,
    ]
}

fn day_cell(current: &NaiveDate, current_month_range: &RangeInclusive<NaiveDate>) -> Node<Msg> {
    div![
        C!["day"],
        attrs![At::Class => format!("{}", current.weekday())],
        if current_month_range.contains(&current) {
            C!["inCurrentMonth"]
        } else {
            C!["outCurrentMonth"]
        },
        format!("{}", current.day()).as_str(),
    ]
}
