use reqwest;
use serde_json;
use crate::objects::Races;

pub fn api_pull(url: &str) -> Vec<Races> {
    let data = reqwest::blocking::get(url)
        .expect("Problem retreiving data, check connection")
        .text()
        .expect("Problem converting to text");
    let data: serde_json::Value = serde_json::from_str(&data).expect("Problem converting data");
    let data = data.get("MRData")
        .expect("Code 100: Problem reading data")
        .get("RaceTable")
        .expect("Code 101: Problem reading data")
        .get("Races")
        .expect("Code 102: Problem reading data");
    // Deserialize
    let data: Vec<Races> = serde_json::from_value(data.clone()).expect("Problem deserializing data");
    data
}
