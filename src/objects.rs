use serde::Deserialize;
use chrono::prelude::{DateTime, Utc};
use crate::time;

#[derive(Deserialize)]
pub struct Races {
    pub season: String,
    pub round: String,
    pub url: String,
    #[serde(rename(deserialize = "raceName"))]
    pub race_name: String,
    #[serde(rename(deserialize = "Circuit"))]
    pub circuit: Circuits,
    pub date: String,
    pub time: String,
    #[serde(rename(deserialize = "FirstPractice"))]
    pub fp1: Option<RaceInfo>,
    #[serde(rename(deserialize = "SecondPractice"))]
    pub fp2: Option<RaceInfo>,
    #[serde(rename(deserialize = "ThirdPractice"))]
    pub fp3: Option<RaceInfo>,
    #[serde(rename(deserialize = "Qualifying"))]
    pub quali: Option<RaceInfo>,
    #[serde(rename(deserialize = "Sprint"))]
    pub sprint: Option<RaceInfo>,
}

#[derive(Deserialize)]
pub struct Circuits {
    #[serde(rename(deserialize = "circuitName"))]
    pub circuit_name: String,
    #[serde(rename(deserialize = "Location"))]
    pub location: Local,
}

#[derive(Deserialize)]
pub struct RaceInfo {
    pub date: String,
    pub time: String,
}

#[derive(Deserialize)]
pub struct Local {
    pub locality: String,
    pub country: String,
}

impl Races {
    pub fn print_info(&self) {
        println!("Season: {}, Round {}", self.season, self.round);
        println!("Race: {}", self.race_name);
        println!("Circuit: {}", self.circuit.circuit_name);
        println!("Location: {}, {}"
                    , self.circuit.location.locality
                    , self.circuit.location.country);
        self.sub_info(&self.fp1, "FP1");
        self.sub_info_verify(&self.fp2, "FP2", &self.sprint, "Sprint Qualifying");
        self.sub_info(&self.sprint, "Sprint");
        self.sub_info(&self.fp3, "FP3");
        self.sub_info(&self.quali, "Qualifying");
        println!("Main Race:\n\tDate: {}", time::to_str_localtz(&time::to_utc(&self.date, &self.time)));
    }

    fn sub_info(&self, info: &Option<RaceInfo>, name: &str) {
        match info {
            Some(ri) => {
                println!("{}:", name);
                self.print_sub(&ri)
            },
            None => (),
        }
    }

    fn sub_info_verify(&self, info: &Option<RaceInfo>, name: &str, verify: &Option<RaceInfo>, alt_name: &str) {
        match verify {
            Some(_) => {self.sub_info(info, alt_name);},
            None => {self.sub_info(info, name);},
        }
    }

    fn print_sub(&self, ri: &RaceInfo) {
        let dt: DateTime<Utc> = time::to_utc(&ri.date, &ri.time);
        println!("\tDate: {}", time::to_str_localtz(&dt));
    }
}
