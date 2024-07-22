mod cli;
use f1s_lib::Races;

fn main() {
    let races = Races::new();
    cli::print_schedule(&races);
    cli::confirm_exit();
}
