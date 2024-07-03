mod cli;

fn main() {
    let races = f1s::Races::from_ergast_json();
    cli::print_schedule(&races);
    cli::confirm_exit()
}
