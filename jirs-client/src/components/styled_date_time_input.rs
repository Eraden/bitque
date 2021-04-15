use {
    crate::{
        components::{
            styled_button::StyledButton, styled_icon::Icon, styled_tooltip::StyledTooltip,
        },
        shared::ToNode,
        FieldId, Msg,
    },
    chrono::prelude::*,
    chrono::Duration,
    seed::{prelude::*, *},
    std::ops::RangeInclusive,
};

use crate::components::styled_button::ButtonVariant;
use crate::components::styled_tooltip::TooltipVariant;

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
        .into_node()
    };

    let header_text = current.format("%B %Y").to_string();

    let tooltip = StyledTooltip {
        visible: values.popup_visible,
        class_list: "",
        children: vec![
            h2![left_action, span![header_text], right_action],
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
        StyledButton {
            on_click: Some(on_focus),
            text: Some(text.as_str()),
            variant: ButtonVariant::Empty,
            ..Default::default()
        }
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
