use chrono::{prelude::{DateTime, Utc}, FixedOffset};

pub fn gmt8_str(time: &DateTime<Utc>) -> String {
    time.with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap())
        .format("%a %Y-%m-%d %H:%M:%S")
        .to_string()
}

pub fn to_utc(race_date: &str, race_time: &str) -> DateTime<Utc> {
    let time: String = format!("{}T{}", race_date, race_time);
    let time = time.parse::<DateTime<Utc>>().expect("Problem converting time");
    time
}
