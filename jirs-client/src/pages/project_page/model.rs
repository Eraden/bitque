use {crate::shared::drag::DragState, jirs_data::*, std::collections::HashMap};

#[derive(Default, Debug)]
pub struct StatusIssueIds {
    pub status_id: IssueStatusId,
    pub status_name: IssueStatusName,
    pub issue_ids: Vec<EpicId>,
}

#[derive(Default, Debug)]
pub struct EpicIssuePerStatus {
    pub epic_ref: Option<(EpicId, EpicName)>,
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
    pub fn visible_issues(
        page: &ProjectPage,
        epics: &[Epic],
        statuses: &[IssueStatus],
        issues: &[Issue],
        user: &Option<User>,
    ) -> Vec<EpicIssuePerStatus> {
        let mut map = vec![];
        let epics = vec![None]
            .into_iter()
            .chain(epics.iter().map(|s| Some((s.id, s.name.as_str()))));

        let statuses = statuses.iter().map(|s| (s.id, s.name.as_str()));

        let mut issues: Vec<&Issue> = issues.iter().collect();
        if page.recently_updated_filter {
            let mut m = HashMap::new();
            let mut sorted = vec![];
            for issue in issues.into_iter() {
                sorted.push((issue.id, issue.updated_at));
                m.insert(issue.id, issue);
            }
            sorted.sort_by(|(_, a_time), (_, b_time)| a_time.cmp(b_time));
            issues = sorted
                .into_iter()
                .take(10)
                .flat_map(|(id, _)| m.remove(&id))
                .collect();
            issues.sort_by(|a, b| a.list_position.cmp(&b.list_position));
        }

        for epic in epics {
            let mut per_epic_map = EpicIssuePerStatus {
                epic_ref: epic.map(|(id, name)| (id, name.to_string())),
                ..Default::default()
            };

            for (current_status_id, issue_status_name) in statuses.to_owned() {
                let mut per_status_map = StatusIssueIds {
                    status_id: current_status_id,
                    status_name: issue_status_name.to_string(),
                    ..Default::default()
                };
                for issue in issues.iter() {
                    if issue.epic_id == epic.map(|(id, _)| id)
                        && issue_filter_status(issue, current_status_id)
                        && issue_filter_with_avatars(issue, &page.active_avatar_filters)
                        && issue_filter_with_text(issue, page.text_filter.as_str())
                        && issue_filter_with_only_my(issue, page.only_my_filter, user)
                    {
                        per_status_map.issue_ids.push(issue.id);
                    }
                }
                per_epic_map.per_status_issues.push(per_status_map);
            }
            map.push(per_epic_map);
        }
        map
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
fn issue_filter_status(issue: &Issue, current_status_id: IssueStatusId) -> bool {
    issue.issue_status_id == current_status_id
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
