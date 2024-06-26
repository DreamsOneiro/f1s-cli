use chrono::prelude::{DateTime, Utc};
use crate::time::to_utc;
use crate::objects::Races;

fn print_title(race_num: usize) {
    if race_num == 1 {
        println!("\nCurrent GP:");
    } else if race_num == 2 {
        println!("\n==========================================");
        println!("\nNext GP:");
    }
}

fn find_schedule(races: &Vec<Races>) -> Option<usize> {
    let time_now: DateTime<Utc> = Utc::now();
    for (i, race) in races.iter().enumerate() {
        let race_time: DateTime<Utc> = to_utc(&race.date, &race.time);
        if time_now < race_time {
            return Some(i);
        }
    }
    None
}

pub fn print_schedule(races: &Vec<Races>) {
    let index = find_schedule(races).expect("Problem finding race");
    print_title(1);
    races[index].print_info();
    print_title(2);
    races[index+1].print_info();
}
