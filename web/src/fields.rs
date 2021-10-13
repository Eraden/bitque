use jirs_data::{
    CommentFieldId, EpicFieldId, InviteFieldId, IssueFieldId, ProjectFieldId, SignInFieldId,
    SignUpFieldId, UsersFieldId,
};

pub type AvatarFilterActive = bool;

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Hash)]
pub enum EditIssueModalSection {
    Issue(IssueFieldId),
    Comment(CommentFieldId),
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum RteField {
    CodeLang(Box<FieldId>),
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Hash)]
#[repr(C)]
pub enum ButtonId {
    JustifyAll,
    JustifyCenter,
    JustifyLeft,
    JustifyRight,
    Undo,
    Redo,
    RemoveFormat,
    Bold,
    Italic,
    Underscore,
    Strikethrough,
    Subscript,
    Superscript,

    Font,
    ListingDots,
    ListingNumber,
    Table,
    Paragraph,
    CodeAlt,
    Indent,
    Outdent,

    CloseRteTableTooltip,
    RteInsertCode,
    RteInjectCode,
    RteInsertTable,
}

impl ButtonId {
    #[inline(always)]
    pub fn to_str<'l>(&self) -> &'l str {
        match self {
            ButtonId::JustifyAll => "justifyAll",
            ButtonId::JustifyCenter => "justifyCenter",
            ButtonId::JustifyLeft => "justifyLeft",
            ButtonId::JustifyRight => "justifyRight",
            ButtonId::Undo => "undo",
            ButtonId::Redo => "redo",

            ButtonId::RemoveFormat => "removeFormat",
            ButtonId::Bold => "bold",
            ButtonId::Italic => "italic ",
            ButtonId::Underscore => "underscore",
            ButtonId::Strikethrough => "strikethrough",
            ButtonId::Subscript => "subscript",
            ButtonId::Superscript => "superscript",

            ButtonId::Font => "font",
            ButtonId::ListingDots => "listingDots",
            ButtonId::ListingNumber => "listingNumber",
            ButtonId::Table => "table",
            ButtonId::Paragraph => "paragraph",
            ButtonId::CodeAlt => "codeAlt",
            ButtonId::Indent => "indent",
            ButtonId::Outdent => "outdent",

            ButtonId::CloseRteTableTooltip => "closeRteTableTooltip",
            ButtonId::RteInsertCode => "rteInsertCode",
            ButtonId::RteInjectCode => "rteInjectCode",
            ButtonId::RteInsertTable => "rteInsertTable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Hash)]
pub enum IssuesAndFiltersId {
    Jql,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum FieldId {
    NoField,
    SignIn(SignInFieldId),
    SignUp(SignUpFieldId),
    Invite(InviteFieldId),
    Users(UsersFieldId),
    Profile(UsersFieldId),
    // issue
    AddIssueModal(IssueFieldId),
    EditIssueModal(EditIssueModalSection),
    IssuesAndFilters(IssuesAndFiltersId),
    // epic
    EditEpic(EpicFieldId),
    // project boards
    TextFilterBoard,
    CopyButtonLabel,

    ProjectSettings(ProjectFieldId),
    Rte(RteField),
}

impl FieldId {
    pub fn to_str(&self) -> &'static str {
        match self {
            FieldId::NoField => "",
            FieldId::EditIssueModal(sub) => match sub {
                EditIssueModalSection::Issue(IssueFieldId::Type) => "issueTypeEditModalTop",
                EditIssueModalSection::Issue(IssueFieldId::Title) => "titleIssueEditModal",
                EditIssueModalSection::Issue(IssueFieldId::Description) => {
                    "descriptionIssueEditModal"
                }
                EditIssueModalSection::Issue(IssueFieldId::IssueStatusId) => "statusIssueEditModal",
                EditIssueModalSection::Issue(IssueFieldId::Assignees) => "assigneesIssueEditModal",
                EditIssueModalSection::Issue(IssueFieldId::Reporter) => "reporterIssueEditModal",
                EditIssueModalSection::Issue(IssueFieldId::Priority) => "priorityIssueEditModal",
                EditIssueModalSection::Issue(IssueFieldId::Estimate) => "estimateIssueEditModal",
                EditIssueModalSection::Issue(IssueFieldId::TimeSpent) => "timeSpendIssueEditModal",
                EditIssueModalSection::Issue(IssueFieldId::TimeRemaining) => {
                    "timeRemainingIssueEditModal"
                }
                EditIssueModalSection::Comment(CommentFieldId::Body) => "editIssue-commentBody",
                EditIssueModalSection::Issue(IssueFieldId::ListPosition) => {
                    "editIssue-listPosition"
                }
                EditIssueModalSection::Issue(IssueFieldId::EpicName) => "editIssue-epicName",
                EditIssueModalSection::Issue(IssueFieldId::EpicStartsAt) => {
                    "editIssue-epicStartsAt"
                }
                EditIssueModalSection::Issue(IssueFieldId::EpicEndsAt) => "editIssue-epicEndsAt",
            },
            FieldId::AddIssueModal(sub) => match sub {
                IssueFieldId::Type => "issueTypeAddIssueModal",
                IssueFieldId::Title => "summaryAddIssueModal",
                IssueFieldId::Description => "descriptionAddIssueModal",
                IssueFieldId::Reporter => "reporterAddIssueModal",
                IssueFieldId::Assignees => "assigneesAddIssueModal",
                IssueFieldId::Priority => "issuePriorityAddIssueModal",
                IssueFieldId::IssueStatusId => "addIssueModal-status",
                IssueFieldId::Estimate => "addIssueModal-estimate",
                IssueFieldId::TimeSpent => "addIssueModal-timeSpend",
                IssueFieldId::TimeRemaining => "addIssueModal-timeRemaining",
                IssueFieldId::ListPosition => "addIssueModal-listPosition",
                IssueFieldId::EpicName => "addIssueModal-epicName",
                IssueFieldId::EpicStartsAt => "addIssueModal-epicStartsAt",
                IssueFieldId::EpicEndsAt => "addIssueModal-epicEndsAt",
            },
            FieldId::TextFilterBoard => "textFilterBoard",
            FieldId::CopyButtonLabel => "copyButtonLabel",
            FieldId::ProjectSettings(sub) => match sub {
                ProjectFieldId::Name => "projectSettings-name",
                ProjectFieldId::Url => "projectSettings-url",
                ProjectFieldId::Description => "projectSettings-description",
                ProjectFieldId::Category => "projectSettings-category",
                ProjectFieldId::TimeTracking => "projectSettings-timeTracking",
                ProjectFieldId::IssueStatusName => "projectSettings-issueStatusName",
                ProjectFieldId::DescriptionMode => "projectSettings-descriptionMode",
            },
            FieldId::SignIn(sub) => match sub {
                SignInFieldId::Email => "login-email",
                SignInFieldId::Username => "login-username",
                SignInFieldId::Token => "login-token",
            },
            FieldId::SignUp(sub) => match sub {
                SignUpFieldId::Username => "signUp-email",
                SignUpFieldId::Email => "signUp-username",
            },
            FieldId::Invite(sub) => match sub {
                InviteFieldId::Token => "invite-token",
            },
            FieldId::Users(sub) => match sub {
                UsersFieldId::Username => "users-username",
                UsersFieldId::Email => "users-email",
                UsersFieldId::UserRole => "users-userRole",
                UsersFieldId::Avatar => "users-avatar",
                UsersFieldId::CurrentProject => "users-currentProject",
                UsersFieldId::TextEditorMode => "users-textEditorMode",
            },
            FieldId::Profile(sub) => match sub {
                UsersFieldId::Username => "profile-username",
                UsersFieldId::Email => "profile-email",
                UsersFieldId::UserRole => "profile-userRole",
                UsersFieldId::Avatar => "profile-avatar",
                UsersFieldId::CurrentProject => "profile-currentProject",
                UsersFieldId::TextEditorMode => "profile-textEditorMode",
            },
            FieldId::EditEpic(sub) => match sub {
                EpicFieldId::Name => "epicEpic-name",
                EpicFieldId::StartsAt => "epicEpic-startsAt",
                EpicFieldId::EndsAt => "epicEpic-endsAt",
                EpicFieldId::TransformInto => "epicEpic-transformInto",
            },
            FieldId::Rte(..) => "rte",
            FieldId::IssuesAndFilters(sub) => match sub {
                IssuesAndFiltersId::Jql => "issuesAndFilters-jql",
            },
        }
    }
}

impl std::fmt::Display for FieldId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str())
    }
}
