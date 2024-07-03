use f1s_lib::{Races, RaceInfo, schedule, time};

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
            println!("Current GP:");
        } else if race_num == 2 {
            println!("====================================\n");
            println!("Next GP:");
        }
    }

    // Print all information of a single race in format
    fn print_rinfo(races: &Races) {
        println!("Season: {}, Round {}", races.season, races.round);
        println!("Race: {}", races.race_name);
        println!("Circuit: {}", races.circuit.circuit_name);
        println!("Location: {}, {}"
            , races.circuit.location.locality
            , races.circuit.location.country);
        print_sub_info(&races.fp1, "FP1");
        print_sub_info_verify(&races.fp2, &races.sprint, "FP2", "SQ"); // Print SQ instead of FP2
        print_sub_info(&races.sprint, "Sprint");
        print_sub_info(&races.fp3, "FP3");
        print_sub_info(&races.quali, "Qualifying");
        println!("Main Race:\n\tDate: {}\n", time::to_str_localtz(&time::to_utc(&races.date, &races.time)));

        // Handle Option<RaceInfo>
        fn print_sub_info(info: &Option<RaceInfo>, name: &str) {
            match info {
                Some(ri) => {
                    println!("{}:", name);
                    print_sub(&ri)
                },
                None => (),
            }

            fn print_sub(ri: &RaceInfo) {
                let dt = time::to_utc(&ri.date, &ri.time);
                println!("\tDate: {}", time::to_str_localtz(&dt));
            }
        }

        // Check if Option contain Some or None before printing
        fn print_sub_info_verify(
            info: &Option<RaceInfo>, 
            switch: &Option<RaceInfo>, 
            name: &str, 
            alt_name: &str
        ) {
            match switch {
                Some(_) => {print_sub_info(info, alt_name);},
                None => {print_sub_info(info, name);},
            }
        }
    }
}

pub fn confirm_exit() {
    if cfg!(windows) {
        println!("Press Enter to quit.");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}
