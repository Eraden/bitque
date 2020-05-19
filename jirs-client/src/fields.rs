use jirs_data::{
    CommentFieldId, InviteFieldId, IssueFieldId, ProjectFieldId, SignInFieldId, SignUpFieldId,
    UsersFieldId,
};

pub type AvatarFilterActive = bool;

#[derive(Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum EditIssueModalSection {
    Issue(IssueFieldId),
    Comment(CommentFieldId),
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum FieldId {
    SignIn(SignInFieldId),
    SignUp(SignUpFieldId),
    Invite(InviteFieldId),
    Users(UsersFieldId),
    Profile(UsersFieldId),
    // issue
    AddIssueModal(IssueFieldId),
    EditIssueModal(EditIssueModalSection),
    // project boards
    TextFilterBoard,
    CopyButtonLabel,

    ProjectSettings(ProjectFieldId),
}

impl std::fmt::Display for FieldId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldId::EditIssueModal(sub) => match sub {
                EditIssueModalSection::Issue(IssueFieldId::Type) => {
                    f.write_str("issueTypeEditModalTop")
                }
                EditIssueModalSection::Issue(IssueFieldId::Title) => {
                    f.write_str("titleIssueEditModal")
                }
                EditIssueModalSection::Issue(IssueFieldId::Description) => {
                    f.write_str("descriptionIssueEditModal")
                }
                EditIssueModalSection::Issue(IssueFieldId::IssueStatusId) => {
                    f.write_str("statusIssueEditModal")
                }
                EditIssueModalSection::Issue(IssueFieldId::Assignees) => {
                    f.write_str("assigneesIssueEditModal")
                }
                EditIssueModalSection::Issue(IssueFieldId::Reporter) => {
                    f.write_str("reporterIssueEditModal")
                }
                EditIssueModalSection::Issue(IssueFieldId::Priority) => {
                    f.write_str("priorityIssueEditModal")
                }
                EditIssueModalSection::Issue(IssueFieldId::Estimate) => {
                    f.write_str("estimateIssueEditModal")
                }
                EditIssueModalSection::Issue(IssueFieldId::TimeSpent) => {
                    f.write_str("timeSpendIssueEditModal")
                }
                EditIssueModalSection::Issue(IssueFieldId::TimeRemaining) => {
                    f.write_str("timeRemainingIssueEditModal")
                }
                EditIssueModalSection::Comment(CommentFieldId::Body) => {
                    f.write_str("editIssue-commentBody")
                }
                EditIssueModalSection::Issue(IssueFieldId::ListPosition) => {
                    f.write_str("editIssue-listPosition")
                }
            },
            FieldId::AddIssueModal(sub) => match sub {
                IssueFieldId::Type => f.write_str("issueTypeAddIssueModal"),
                IssueFieldId::Title => f.write_str("summaryAddIssueModal"),
                IssueFieldId::Description => f.write_str("descriptionAddIssueModal"),
                IssueFieldId::Reporter => f.write_str("reporterAddIssueModal"),
                IssueFieldId::Assignees => f.write_str("assigneesAddIssueModal"),
                IssueFieldId::Priority => f.write_str("issuePriorityAddIssueModal"),
                IssueFieldId::IssueStatusId => f.write_str("addIssueModal-status"),
                IssueFieldId::Estimate => f.write_str("addIssueModal-estimate"),
                IssueFieldId::TimeSpent => f.write_str("addIssueModal-timeSpend"),
                IssueFieldId::TimeRemaining => f.write_str("addIssueModal-timeRemaining"),
                IssueFieldId::ListPosition => f.write_str("addIssueModal-listPosition"),
            },
            FieldId::TextFilterBoard => f.write_str("textFilterBoard"),
            FieldId::CopyButtonLabel => f.write_str("copyButtonLabel"),
            FieldId::ProjectSettings(sub) => match sub {
                ProjectFieldId::Name => f.write_str("projectSettings-name"),
                ProjectFieldId::Url => f.write_str("projectSettings-url"),
                ProjectFieldId::Description => f.write_str("projectSettings-description"),
                ProjectFieldId::Category => f.write_str("projectSettings-category"),
                ProjectFieldId::TimeTracking => f.write_str("projectSettings-timeTracking"),
                ProjectFieldId::IssueStatusName => f.write_str("projectSettings-issueStatusName"),
            },
            FieldId::SignIn(sub) => match sub {
                SignInFieldId::Email => f.write_str("login-email"),
                SignInFieldId::Username => f.write_str("login-username"),
                SignInFieldId::Token => f.write_str("login-token"),
            },
            FieldId::SignUp(sub) => match sub {
                SignUpFieldId::Username => f.write_str("signUp-email"),
                SignUpFieldId::Email => f.write_str("signUp-username"),
            },
            FieldId::Invite(sub) => match sub {
                InviteFieldId::Token => f.write_str("invite-token"),
            },
            FieldId::Users(sub) => match sub {
                UsersFieldId::Username => f.write_str("users-username"),
                UsersFieldId::Email => f.write_str("users-email"),
                UsersFieldId::UserRole => f.write_str("users-userRole"),
                UsersFieldId::Avatar => f.write_str("users-avatar"),
            },
            FieldId::Profile(sub) => match sub {
                UsersFieldId::Username => f.write_str("profile-username"),
                UsersFieldId::Email => f.write_str("profile-email"),
                UsersFieldId::UserRole => f.write_str("profile-userRole"),
                UsersFieldId::Avatar => f.write_str("profile-avatar"),
            },
        }
    }
}
