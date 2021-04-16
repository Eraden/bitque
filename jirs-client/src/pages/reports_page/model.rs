use chrono::prelude::*;
use chrono::NaiveDate;

#[derive(Debug)]
pub struct ReportsPage {
    pub selected_day: Option<chrono::NaiveDate>,
    pub hovered_day: Option<chrono::NaiveDate>,
    pub first_day: NaiveDate,
    pub last_day: NaiveDate,
}

impl Default for ReportsPage {
    fn default() -> Self {
        let first_day = chrono::Utc::today().with_day(1).unwrap().naive_local();
        let last_day = (first_day + chrono::Duration::days(32))
            .with_day(1)
            .unwrap()
            - chrono::Duration::days(1);
        Self {
            first_day,
            last_day,
            selected_day: None,
            hovered_day: None,
        }
    }
}
