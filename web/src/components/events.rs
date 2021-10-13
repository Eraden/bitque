use std::str::FromStr;

use seed::prelude::{ev, Ev, EventHandler};

use crate::components::styled_date_time_input::StyledDateTimeChanged;
use crate::components::styled_md_editor::MdEditorMode;
use crate::components::styled_select::StyledSelectChanged;
use crate::{resolve_page, FieldChange};

type EvHandler = EventHandler<crate::Msg>;

pub fn on_click_change_number_input(field_id: crate::FieldId, value: u32) -> EvHandler {
    ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        ev.prevent_default();

        crate::Msg::U32InputChanged(field_id, value)
    })
}

pub fn on_change_set_str_input(field_id: crate::FieldId) -> EvHandler {
    ev(Ev::Change, move |ev| {
        ev.stop_propagation();

        let target = ev.target().unwrap();
        crate::Msg::StrInputChanged(field_id, seed::to_input(&target).value())
    })
}

pub fn on_click_drop_modal() -> EvHandler {
    ev(Ev::Click, move |ev| {
        ev.stop_propagation();

        crate::Msg::ModalDropped
    })
}

pub fn on_click_change_day(field_id: crate::FieldId, date: chrono::NaiveDateTime) -> EvHandler {
    ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        ev.prevent_default();

        // log::info!("{:?}", date);
        crate::Msg::StyledDateTimeInputChanged(
            field_id,
            StyledDateTimeChanged::DayChanged(Some(date)),
        )
    })
}

pub fn on_click_change_date_time_visibility(field_id: crate::FieldId, visible: bool) -> EvHandler {
    ev(Ev::Click, move |ev| {
        ev.prevent_default();
        ev.stop_propagation();

        crate::Msg::StyledDateTimeInputChanged(
            field_id,
            StyledDateTimeChanged::PopupVisibilityChanged(!visible),
        )
    })
}

pub fn on_click_change_month(field_id: crate::FieldId, date: chrono::NaiveDateTime) -> EvHandler {
    ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        ev.prevent_default();

        crate::Msg::StyledDateTimeInputChanged(
            field_id,
            StyledDateTimeChanged::MonthChanged(Some(date)),
        )
    })
}

pub fn on_change_image_input(field_id: crate::FieldId) -> EvHandler {
    ev(Ev::Change, move |ev| {
        let target = ev.target().unwrap();
        let input = seed::to_input(&target);
        let v = input
            .files()
            .map(|list| {
                (0..list.length())
                    .filter_map(|i| list.get(i))
                    .collect::<Vec<web_sys::File>>()
            })
            .unwrap_or_default();
        crate::Msg::FileInputChanged(field_id, v)
    })
}

pub fn on_keyup_change_select_text(field_id: crate::FieldId) -> EvHandler {
    ev(Ev::KeyUp, move |ev| {
        ev.stop_propagation();

        let target = ev.target().unwrap();
        let value = seed::to_input(&target).value();
        crate::Msg::StyledSelectChanged(field_id, StyledSelectChanged::Text(value))
    })
}

pub fn on_click_change_select_dropdown_visibility(
    field_id: crate::FieldId,
    opened: bool,
) -> EvHandler {
    ev(Ev::Click, move |ev| {
        ev.stop_propagation();

        crate::Msg::StyledSelectChanged(field_id, StyledSelectChanged::DropDownVisibility(!opened))
    })
}

pub fn on_click_change_select_selected(field_id: crate::FieldId, value: Option<u32>) -> EvHandler {
    ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        ev.prevent_default();
        crate::Msg::StyledSelectChanged(field_id, StyledSelectChanged::Changed(value))
    })
}

pub fn on_click_change_select_remove_multi(field_id: crate::FieldId, value: u32) -> EvHandler {
    ev(Ev::Click, move |ev| {
        ev.stop_propagation();
        crate::Msg::StyledSelectChanged(field_id, StyledSelectChanged::RemoveMulti(value))
    })
}

pub fn on_click_change_page(href: String) -> EvHandler {
    ev(Ev::Click, move |ev| {
        if href.starts_with('/') {
            ev.prevent_default();
            ev.stop_propagation();

            if let Ok(url) = seed::Url::from_str(href.as_str()) {
                url.go_and_push();
                return resolve_page(url).map(crate::Msg::ChangePage);
            }
        }

        None as Option<crate::Msg>
    })
}

pub fn on_click_change_tab(field_id: crate::FieldId, new_mode: MdEditorMode) -> EvHandler {
    ev(Ev::Click, move |ev| {
        ev.stop_propagation();

        crate::Msg::ModalChanged(FieldChange::TabChanged(field_id, new_mode))
    })
}

pub fn on_click_noop() -> EvHandler {
    noop(Ev::Click)
}

pub fn on_keyup_noop() -> EvHandler {
    noop(Ev::KeyUp)
}

fn noop(event: Ev) -> EvHandler {
    ev(event, |ev| {
        ev.stop_propagation();
        ev.prevent_default();
        None as Option<crate::Msg>
    })
}

pub fn on_event_change_text_value(
    field_id: crate::FieldId,
    update_event: Ev,
    handler_disable_auto_resize: bool,
) -> EvHandler {
    ev(update_event, move |event| {
        event.stop_propagation();

        let value = event
            .target()
            .map(|target| seed::to_textarea(&target).value())
            .unwrap_or_default();

        if handler_disable_auto_resize && value.contains('\n') {
            event.prevent_default();
        }

        crate::Msg::StrInputChanged(
            field_id,
            if handler_disable_auto_resize {
                value.trim().to_string()
            } else {
                value
            },
        )
    })
}
