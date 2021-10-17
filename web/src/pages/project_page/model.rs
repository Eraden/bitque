use std::collections::HashMap;

use chrono::NaiveDateTime;
use jirs_data::*;

use crate::shared::drag::DragState;

#[derive(Default, Debug)]
pub struct StatusIssueIds {
    pub status_id: IssueStatusId,
    pub status_name: IssueStatusName,
    pub issue_ids: Vec<IssueId>,
}

#[derive(Default, Debug)]
pub struct EpicIssuePerStatus {
    pub epic_ref: Option<(EpicId, EpicName, Option<StartsAt>, Option<EndsAt>)>,
    pub per_status_issues: Vec<StatusIssueIds>,
}

#[derive(Debug, Default)]
pub struct ProjectPage {
    pub text_filter: String,
    pub active_avatar_filters: Vec<UserId>,
    pub only_my_filter: bool,
    pub recently_updated_filter: bool,
    pub issue_drag: DragState,
    pub visible_issues: Vec<EpicIssuePerStatus>,
}

impl ProjectPage {
    pub fn visible_issues<'model, IssueIter, IssueStatusIter, EpicIter>(
        page: &ProjectPage,
        num_of_epics: usize,
        epics: EpicIter,
        num_of_statuses: usize,
        statuses: IssueStatusIter,
        issues: IssueIter,
        user: &Option<User>,
    ) -> Vec<EpicIssuePerStatus>
    where
        IssueIter: std::iter::Iterator<Item = &'model Issue>,
        IssueStatusIter: std::iter::Iterator<Item = &'model IssueStatus>,
        EpicIter: std::iter::Iterator<Item = &'model Epic>,
    {
        let mut scoped_issues = {
            let issues = issues.filter(|issue| {
                issue_filter_with_avatars(issue, &page.active_avatar_filters)
                    && issue_filter_with_text(issue, page.text_filter.as_str())
                    && issue_filter_with_only_my(issue, page.only_my_filter, user)
            });
            if page.recently_updated_filter {
                let mut m = HashMap::with_capacity(num_of_statuses * num_of_epics);
                let mut sorted: Vec<(IssueId, NaiveDateTime)> = issues
                    .map(|issue| {
                        m.insert(issue.id, issue);
                        (issue.id, issue.updated_at)
                    })
                    .collect();
                sorted.sort_by(|(_, a_time), (_, b_time)| a_time.cmp(b_time));
                to_per_epic_and_status_scoped(
                    num_of_epics,
                    sorted
                        .into_iter()
                        .take(10)
                        .flat_map(|(id, _)| m.remove(&id)),
                )
            } else {
                to_per_epic_and_status_scoped(num_of_epics, issues)
            }
        };

        let statuses = statuses
            .map(|s| (s.id, s.name.as_str()))
            .collect::<Vec<(IssueStatusId, &str)>>();

        vec![None]
            .into_iter()
            .chain(
                epics.map(|epic| Some((epic.id, epic.name.as_str(), epic.starts_at, epic.ends_at))),
            )
            .map(|epic| EpicIssuePerStatus {
                epic_ref: epic.map(|(id, name, starts_at, ends_at)| {
                    (id, name.to_string(), starts_at, ends_at)
                }),
                per_status_issues: statuses
                    .iter()
                    .map(|(current_status_id, issue_status_name)| StatusIssueIds {
                        status_id: *current_status_id,
                        status_name: issue_status_name.to_string(),
                        issue_ids: scoped_issues
                            .remove(&(epic.map(|(id, ..)| id), *current_status_id))
                            .map(|mut v| {
                                v.sort_by(|a, b| a.list_position.cmp(&b.list_position));
                                v.into_iter().map(|issue| issue.id).collect()
                            })
                            .unwrap_or_default(),
                    })
                    .collect(),
            })
            .collect()
    }
}

#[inline]
fn issue_filter_with_avatars(issue: &Issue, user_ids: &[UserId]) -> bool {
    if user_ids.is_empty() {
        return true;
    }
    user_ids.contains(&issue.reporter_id) || issue.user_ids.iter().any(|id| user_ids.contains(id))
}

#[inline]
fn issue_filter_with_text(issue: &Issue, text: &str) -> bool {
    text.is_empty() || issue.title.contains(text)
}

#[inline]
fn issue_filter_with_only_my(issue: &Issue, only_my: bool, user: &Option<User>) -> bool {
    let my_id = user.as_ref().map(|u| u.id).unwrap_or_default();
    !only_my || issue.user_ids.contains(&my_id)
}

fn to_per_epic_and_status_scoped<'model, IssueIter>(
    num_of_epics: usize,
    issues: IssueIter,
) -> HashMap<(Option<EpicId>, IssueStatusId), Vec<&'model Issue>>
where
    IssueIter: std::iter::Iterator<Item = &'model Issue>,
{
    issues.fold(HashMap::with_capacity(num_of_epics), |mut m, issue| {
        m.entry((issue.epic_id, issue.issue_status_id))
            .or_insert_with(|| Vec::with_capacity(100))
            .push(issue);
        m
    })
}
