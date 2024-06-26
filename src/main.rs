mod objects;
mod api;
mod time;
mod schedule;

use std::io;
use api::api_pull;
use schedule::*;

fn confirm_exit() {
    println!("\nPress Enter to quit.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn main() {
    let request_url = "http://ergast.com/api/f1/current.json";
    let races = api_pull(&request_url);
    print_schedule(&races);
    confirm_exit();
}
