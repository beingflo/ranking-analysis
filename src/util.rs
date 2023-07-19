use std::{collections::HashSet, fs};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Match {
    #[serde(rename = "gameId")]
    pub game_id: String,
    pub players: Vec<String>,
    // First entry in array is offensive player
    #[serde(rename = "teamA")]
    pub team_a: Vec<String>,
    // First entry in array is offensive player
    #[serde(rename = "teamB")]
    pub team_b: Vec<String>,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    #[serde(rename = "teamAScore")]
    pub team_a_score: u32,
    #[serde(rename = "teamBScore")]
    pub team_b_score: u32,
    pub goals: Goals,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Goals {
    pub scorer: Vec<String>,
    pub time: Vec<String>,
}

pub fn read_matches() -> Vec<Match> {
    let data = fs::read_to_string("./src/matches.json").expect("File could not be read");
    let matches: Vec<Match> = serde_json::from_str(&data).expect("Could not serialize data");

    matches
}

pub fn get_players(matches: &Vec<Match>) -> Vec<String> {
    let mut players: HashSet<String> = HashSet::new();

    for m in matches.iter() {
        for p in m.team_a.iter() {
            players.insert(p.into());
        }
        for p in m.team_b.iter() {
            players.insert(p.into());
        }
    }

    players.into_iter().collect()
}
