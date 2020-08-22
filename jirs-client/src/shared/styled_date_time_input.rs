use std::ops::RangeInclusive;

use chrono::Duration;

use {
    chrono::prelude::*,
    seed::{prelude::*, *},
};

use crate::shared::styled_button::StyledButton;
use crate::shared::styled_icon::Icon;
use crate::shared::styled_select::StyledSelect;
use crate::shared::styled_tooltip::StyledTooltip;
use crate::shared::ToChild;
use crate::{shared::ToNode, FieldId, Msg};

#[derive(Debug)]
pub enum StyledDateTimeChanged {
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
    let timestamp = values
        .timestamp
        .unwrap_or_else(|| chrono::Utc::now().naive_utc());
    let start = timestamp.with_day0(0).unwrap();
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

        current_week.push(day_cell(
            &values.field_id,
            &timestamp,
            &current,
            &current_month_range,
        ));

        current += Duration::days(1);
    }
    if !current_week.is_empty() {
        weeks.push(div![C!["week"], current_week]);
    }

    let left_action = {
        let field_id = values.field_id.clone();
        let current = timestamp;
        let on_click_left = mouse_ev(Ev::Click, move |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            let last_day_of_prev_month = current.with_day0(0).unwrap() - Duration::days(1);

            let date = last_day_of_prev_month
                .with_day0(timestamp.day0())
                .unwrap_or_else(|| last_day_of_prev_month);
            Msg::StyledDateTimeInputChanged(
                field_id,
                StyledDateTimeChanged::MonthChanged(Some(date)),
            )
        });
        StyledButton::build()
            .on_click(on_click_left)
            .icon(Icon::DoubleLeft)
            .empty()
            .build()
            .into_node()
    };
    let right_action = {
        let field_id = values.field_id.clone();
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
                .unwrap_or_else(|| last_day_of_next_month);
            Msg::StyledDateTimeInputChanged(
                field_id,
                StyledDateTimeChanged::MonthChanged(Some(date)),
            )
        });
        StyledButton::build()
            .on_click(on_click_right)
            .icon(Icon::DoubleRight)
            .empty()
            .build()
            .into_node()
    };

    let month_select = {
        use num_traits::FromPrimitive;
        let field_id = values.field_id.clone();
        let selected_month = Month::from_u32(current.month()).unwrap_or_else(|| Month::January);

        StyledSelect::build()
            .options(
                vec![
                    Month::January,
                    Month::February,
                    Month::March,
                    Month::April,
                    Month::May,
                    Month::June,
                    Month::July,
                    Month::August,
                    Month::September,
                    Month::October,
                    Month::November,
                    Month::December,
                ]
                .into_iter()
                .map(|month| (month.name().to_string(), month.number_from_month()).to_child())
                .collect(),
            )
            .selected(vec![(
                selected_month.name().to_string(),
                selected_month.number_from_month(),
            )
                .to_child()])
            .build(field_id)
            .into_node()
    };

    let year_select = {
        let field_id = values.field_id.clone();
        let selected_year = current.year();
        StyledSelect::build()
            .options(
                (1980..=Utc::today().year())
                    .into_iter()
                    .map(|i| (i as u32).to_child())
                    .collect(),
            )
            .selected(vec![(selected_year as u32).to_child()])
            .build(field_id)
            .into_node()
    };

    let tooltip = StyledTooltip::build()
        .visible(values.popup_visible)
        .date_time_picker()
        .add_child(h2![
            left_action,
            span![C!["headerText"], month_select, year_select],
            right_action
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
        let visible = values.popup_visible;
        let on_focus = ev(Ev::Click, move |ev| {
            ev.prevent_default();
            ev.stop_propagation();
            Msg::StyledDateTimeInputChanged(
                field_id,
                StyledDateTimeChanged::PopupVisibilityChanged(!visible),
            )
        });
        let text = values
            .timestamp
            .unwrap_or_else(|| Utc::now().naive_utc())
            .date()
            .format("%d/%m/%Y")
            .to_string();
        StyledButton::build()
            .on_click(on_focus)
            .text(text)
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

fn day_cell(
    field_id: &FieldId,
    timestamp: &NaiveDateTime,
    current: &NaiveDateTime,
    current_month_range: &RangeInclusive<NaiveDateTime>,
) -> Node<Msg> {
    let selected_day_class = if *timestamp == *current {
        "selected"
    } else {
        ""
    };

    let on_click = {
        let field_id = field_id.clone();
        let date = *current;
        ev(Ev::Click, move |ev| {
            ev.stop_propagation();
            ev.prevent_default();
            Msg::StyledDateTimeInputChanged(field_id, StyledDateTimeChanged::DayChanged(Some(date)))
        })
    };
    div![
        C!["day"],
        attrs![At::Class => format!("{}", current.weekday())],
        if current_month_range.contains(&current) {
            C!["inCurrentMonth"]
        } else {
            C!["outCurrentMonth"]
        },
        C![selected_day_class],
        format!("{}", current.day()).as_str(),
        on_click,
    ]
}
