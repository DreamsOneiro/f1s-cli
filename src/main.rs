mod objects;
mod api;
mod time;

use std::io;

use objects::Races;
use chrono::prelude::{DateTime, Utc};
use api::api_pull;
use time::to_utc;

fn print_title(status: &bool) {
    if *status {
        println!("\n==========================================\n\nNext GP:");
    } else {
        println!("\nCurrent GP:");
    }
}

fn main() {
    let request_url = "http://ergast.com/api/f1/current.json";
    let races = api_pull(&request_url);
    let time_now: DateTime<Utc> = Utc::now();
    let mut next_race: bool = false;
    for race in races {
        let race_time: DateTime<Utc> = to_utc(&race.date, &race.time);
        if time_now < race_time {
            print_title(&next_race);
            race.print_info();
            if next_race {
                break;
            }
            next_race = true;
            continue;
        } else {
            continue;
        };
    };
    println!("\nPress Enter to quit.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Crash and burn");
}
