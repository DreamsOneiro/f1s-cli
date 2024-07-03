mod cli;
use f1s_lib::Races;

fn main() {
    let races = Races::from_ergast_json();
    cli::print_schedule(&races);
    cli::confirm_exit()
}
