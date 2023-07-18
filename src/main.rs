use std::fs;
use serde_json;
use util::Match;

pub mod util;

fn main() {
    let data = fs::read_to_string("./src/matches.json").expect("File could not be read");
    let matches: Vec<Match> = serde_json::from_str(&data).expect("Could not serialize data");

    println!("{}", matches.len());
}
