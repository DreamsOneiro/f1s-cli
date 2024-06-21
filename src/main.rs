use reqwest;
use serde_json;
use serde::Deserialize;

#[derive(Deserialize)]
struct Races {
    season: String,
    round: String,
    url: String,
    #[serde(rename(deserialize = "raceName"))]
    race_name: String,
    #[serde(rename(deserialize = "Circuit"))]
    circuit: Circuits,
    date: String,
    time: String,
    #[serde(rename(deserialize = "FirstPractice"))]
    fp1: Option<RaceInfo>,
    #[serde(rename(deserialize = "SecondPractice"))]
    fp2: Option<RaceInfo>,
    #[serde(rename(deserialize = "ThirdPractice"))]
    fp3: Option<RaceInfo>,
    #[serde(rename(deserialize = "Qualifying"))]
    quali: Option<RaceInfo>,
}

#[derive(Deserialize)]
struct Circuits {
    #[serde(rename(deserialize = "circuitName"))]
    circuit_name: String,
}

#[derive(Deserialize)]
struct RaceInfo {
    date: String,
    time: String,
}

fn empty_ri() -> RaceInfo {
    RaceInfo {
        date: String::from("None"),
        time: String::from("None"),
    }
}

fn main() {
    let request_url = "http://ergast.com/api/f1/current.json";
    let data = reqwest::blocking::get(request_url)
                .expect("Problem retreiving data")
                .text()
                .expect("Problem converting to text");
    let result: serde_json::Value = serde_json::from_str(&data).expect("Problem converting data");
    let races = result.get("MRData")
                .expect("Code 100: Problem reading data")
                .get("RaceTable")
                .expect("Code 101: Problem reading data")
                .get("Races")
                .expect("Code 102: Problem reading data");
    // print!("{}", races[0]);
    let races: Vec<Races> = serde_json::from_value(races.clone()).unwrap();
    let empty = empty_ri();
    for race in races {
        let message = match &race.fp1 {
            Some(info) => info,
            None => &empty
        };
        println!("{}: {}",race.race_name, message.date)
    };
}
