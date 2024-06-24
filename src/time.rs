use chrono::{prelude::{DateTime, Utc}, FixedOffset, Local};

pub fn gmt8_str(time: &DateTime<Utc>) -> String {
    time.with_timezone(&FixedOffset::east_opt(get_timezone()).unwrap())
        .format("%b %e, %a | %I:%M%p")
        .to_string()
}

pub fn to_utc(race_date: &str, race_time: &str) -> DateTime<Utc> {
    let time: String = format!("{}T{}", race_date, race_time);
    let time = time.parse::<DateTime<Utc>>().expect("Problem converting time");
    time
}

fn get_timezone() -> i32{
    let offset_in_sec: i32 = Local::now().offset().local_minus_utc();
    offset_in_sec
}
