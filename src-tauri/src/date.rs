use chrono::{Datelike, Days, Local, NaiveDate};

pub struct DateRange {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

/// 计算各预设周期对应的起止日期（周一为一周起始）
pub fn compute_range(kind: &str) -> DateRange {
    let today = Local::now().date_naive();
    match kind {
        "today" => DateRange { start: today, end: today },
        "week" => {
            let monday = today - Days::new(today.weekday().num_days_from_monday() as u64);
            DateRange { start: monday, end: monday + Days::new(6) }
        }
        "last_week" => {
            let this_monday = today - Days::new(today.weekday().num_days_from_monday() as u64);
            let last_monday = this_monday - Days::new(7);
            DateRange { start: last_monday, end: last_monday + Days::new(6) }
        }
        "month" => {
            let first = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
            DateRange { start: first, end: month_end(first) }
        }
        "last_month" => {
            let (y, m) = if today.month() == 1 {
                (today.year() - 1, 12)
            } else {
                (today.year(), today.month() - 1)
            };
            let first = NaiveDate::from_ymd_opt(y, m, 1).unwrap();
            DateRange { start: first, end: month_end(first) }
        }
        _ => DateRange { start: today, end: today },
    }
}

fn month_end(first: NaiveDate) -> NaiveDate {
    let (y, m) = if first.month() == 12 {
        (first.year() + 1, 1)
    } else {
        (first.year(), first.month() + 1)
    };
    NaiveDate::from_ymd_opt(y, m, 1).unwrap() - Days::new(1)
}
