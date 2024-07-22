use f1s_lib::{Races, schedule};
use f1s_lib::time::to_str_localtz;

// Print full schedule in CLI
pub fn print_schedule(races: &Vec<Races>) {
    let index = schedule::search_current(races).expect("Problem finding race");
    let limit = races.len();
    print_title(1);
    print_rinfo(&races[index]);
    // Print next race
    if (index + 1) < limit {
        print_title(2);
        print_rinfo(&races[index + 1]);
    }

    fn print_title(race_num: usize) {
        if race_num == 1 {
            println!("--------------");
            println!("| Current GP |");
            println!("--------------");
        } else if race_num == 2 {
            println!("==================================\n");
            println!("-----------");
            println!("| Next GP |");
            println!("-----------");
        }
    }

    // Print all information of a single race in format
    fn print_rinfo(races: &Races) {
        println!("Season: {}, Round {}", races.year, races.round);
        println!("Race: {}", races.grand_prix);
        println!("Circuit: {}", races.circuit);
        println!("Location: {}, {}" , races.locality , races.country);
        println!("----------------------------------");
        println!("FP1:\t{}", to_str_localtz(&races.fp1()));
        if races.has_sprint() {
            println!("SQ:\t{}", to_str_localtz(&races.sq()));
            println!("Sprint:\t{}", to_str_localtz(&races.sprint()));
        } else {
            println!("FP2:\t{}", to_str_localtz(&races.fp2()));
            println!("FP3:\t{}", to_str_localtz(&races.fp3()));
        }
        println!("Quali:\t{}", to_str_localtz(&races.quali()));
        println!("Race:\t{}\n", to_str_localtz(&races.main_race()));
    }
}

pub fn confirm_exit() {
    if cfg!(windows) {
        println!("Press Enter to quit.");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}
