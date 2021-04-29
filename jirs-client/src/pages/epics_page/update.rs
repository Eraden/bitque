use seed::app::Orders;

use crate::model::{Model, Page, PageContent};
use crate::pages::epics_page::EpicsPage;
use crate::ws::board_load;
use crate::{Msg, OperationKind, ResourceKind};

pub fn update(msg: Msg, model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
    if model.user.is_none() {
        return;
    }

    if matches!(model.page, Page::IssuesAndFilters)
        && !matches!(model.page_content, PageContent::IssuesAndFilters(..))
    {
        build_page_content(model);
    }

    match msg {
        Msg::ResourceChanged(ResourceKind::Auth, OperationKind::SingleLoaded, Some(_))
        | Msg::ChangePage(Page::Epics) => {
            board_load(model, orders);
            build_page_content(model);
        }
        Msg::ResourceChanged(ResourceKind::Issue, OperationKind::ListLoaded, ..) => {
            let hash = EpicsPage::build_issues_per_epic(model);
            crate::match_page_mut!(model, Epics).issues_per_epic = hash;
        }
        _ => (),
    }
}

fn build_page_content(model: &mut Model) {
    if matches!(model.page_content, PageContent::Epics(..)) {
        return;
    }
    model.page_content = PageContent::Epics(Box::new(super::EpicsPage::new(model)));
}
