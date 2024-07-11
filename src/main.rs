mod cli;
use f1s_lib::Race;

fn main() {
    let races = Race::new();
    cli::print_schedule(&races);
    cli::confirm_exit()
}
