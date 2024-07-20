use chrono::{Datelike, Local, NaiveDate};

// strが日付のフォーマットに準拠しているかチェックする(ex: 2024-07-20)
pub fn validate_date_str(date_str: &str) -> Result<NaiveDate, String> {
    let today = Local::now().naive_local().date();
    match date_str {
        "today" => Ok(today),
        "eow" => Ok(last_day_of_week(today)),
        "eom" => Ok(last_day_of_month(today)),
        _ => match date_str.parse::<NaiveDate>() {
            Ok(naive_date) => Ok(naive_date),
            Err(_e) => Err("The date value is invalid format, not yyyy-mm-dd.".to_string()),
        },
    }
}

fn last_day_of_week(day: NaiveDate) -> NaiveDate {
    day.week(chrono::Weekday::Sat).last_day()
}

fn last_day_of_month(day: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(day.year(), day.month() + 1, 1)
        .unwrap_or(NaiveDate::from_ymd_opt(day.year() + 1, 1, 1).unwrap())
        .pred_opt()
        .expect("The end of this month does always exist")
}
