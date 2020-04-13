use seed::{prelude::*, *};

use crate::shared::styled_editor::StyledEditor;
use crate::shared::styled_field::StyledField;
use crate::shared::styled_form::StyledForm;
use crate::shared::styled_input::StyledInput;
use crate::shared::styled_select::StyledSelect;
use crate::shared::{inner_layout, ToNode};
use crate::{model, FieldId, Msg, ProjectSettingsFieldId};

pub fn update(_msg: Msg, _model: &mut model::Model, _orders: &mut impl Orders<Msg>) {}

pub fn view(model: &model::Model) -> Node<Msg> {
    let name = StyledInput::build(FieldId::ProjectSettings(ProjectSettingsFieldId::Name))
        .valid(true)
        .build()
        .into_node();
    let name_field = StyledField::build()
        .label("Name")
        .input(name)
        .tip("")
        .build()
        .into_node();

    let url = StyledInput::build(FieldId::ProjectSettings(ProjectSettingsFieldId::Url))
        .valid(true)
        .build()
        .into_node();
    let url_field = StyledField::build()
        .label("Url")
        .input(url)
        .tip("")
        .build()
        .into_node();

    let description = StyledEditor::build(FieldId::ProjectSettings(
        ProjectSettingsFieldId::Description,
    ))
    .text("")
    .update_on(Ev::Change)
    .build()
    .into_node();
    let description_field = StyledField::build()
        .input(description)
        .label("Description")
        .tip("Describe the project in as much detail as you'd like.")
        .build()
        .into_node();

    let form = StyledForm::build()
        .heading("Project Details")
        .add_field(name_field)
        .add_field(url_field)
        .add_field(description_field)
        .build()
        .into_node();

    let project_section = vec![form];

    inner_layout(model, "projectSettings", project_section, empty![])
}
