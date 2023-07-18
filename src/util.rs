use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Match {
	#[serde(rename = "gameId")]
	game_id: String,
	players: Vec<String>,
	// First entry in array is offensive player
	#[serde(rename = "teamA")]
	team_a: Vec<String>,
	// First entry in array is offensive player
	#[serde(rename = "teamB")]
	team_b: Vec<String>,
	#[serde(rename = "startDate")]
	start_date: String,
	#[serde(rename = "endDate")]
	end_date: String,
	#[serde(rename = "teamAScore")]
	team_a_score: u32,
	#[serde(rename = "teamBScore")]
	team_b_score: u32,
	goals: Goals,
}

#[derive(Deserialize, Debug)]
pub struct Goals {
  scorer: Vec<String>,
  time: Vec<String>,
}
