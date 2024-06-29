mod objects;
mod time;
mod api;
mod schedule;

fn confirm_exit() {
    println!("Press Enter to quit.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}

fn main() {
    let request_url = "http://ergast.com/api/f1/current.json";
    let races = api::api_pull(&request_url);
    schedule::print_schedule(&races);
    if cfg!(windows) {
        confirm_exit();
    }
}
