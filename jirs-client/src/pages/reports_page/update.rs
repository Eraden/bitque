use {
    crate::{
        changes::{PageChanged, ReportsPageChange},
        model::{Model, Page, PageContent},
        pages::reports_page::model::ReportsPage,
        ws::board_load,
        Msg, OperationKind, ResourceKind,
    },
    seed::prelude::*,
};

pub fn update(msg: Msg, model: &mut crate::model::Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangePage(Page::Reports) => build_page_content(model),
        Msg::ResourceChanged(ResourceKind::Auth, OperationKind::SingleLoaded, _)
            if model.page == Page::Reports =>
        {
            build_page_content(model);
        }
        _ => {}
    };

    let page = match &mut model.page_content {
        PageContent::Reports(page) => page,
        _ => return,
    };

    if model.user.is_none() {
        return;
    }
    match msg {
        Msg::UserChanged(Some(..))
        | Msg::ResourceChanged(ResourceKind::Auth, OperationKind::SingleLoaded, _)
        | Msg::ChangePage(Page::Reports) => {
            board_load(model, orders);
        }
        Msg::PageChanged(PageChanged::Reports(ReportsPageChange::DayHovered(v))) => {
            page.hovered_day = v;
        }
        Msg::PageChanged(PageChanged::Reports(ReportsPageChange::DaySelected(v))) => {
            page.selected_day = v;
        }
        _ => {}
    }
}

pub fn build_page_content(model: &mut Model) {
    model.page_content = PageContent::Reports(Box::new(ReportsPage::default()))
}
