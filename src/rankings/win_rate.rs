use std::collections::HashMap;

use crate::util::{get_players, Match};

pub struct WinRate {
    num_matches: u32,
    num_wins: u32,
}

pub struct WinRateRanking {
    pub winrates: HashMap<String, WinRate>,
    pub winrate_history: HashMap<String, Vec<(String, f32)>>,
}

pub fn compute_ranking(matches: &Vec<Match>) -> WinRateRanking {
    let mut ranking = WinRateRanking {
        winrates: HashMap::new(),
        winrate_history: HashMap::new(),
    };

    for p in get_players(&matches) {
        ranking.winrates.insert(
            p.clone(),
            WinRate {
                num_matches: 0,
                num_wins: 0,
            },
        );
        ranking.winrate_history.insert(p, Vec::new());
    }

    for m in matches.iter() {
        let all_players = m.team_a.iter().chain(m.team_b.iter());
        let winning_team = if m.team_a_score > m.team_b_score {
            &m.team_a
        } else {
            &m.team_b
        };

        for p in winning_team.iter() {
            ranking.winrates.get_mut(p).unwrap().num_wins += 1;
        }
        for p in all_players {
            ranking.winrates.get_mut(p).unwrap().num_matches += 1;
            ranking.winrate_history.get_mut(p).unwrap().push((
                m.start_date.clone(),
                ranking.winrates.get(p).unwrap().get_winrate(),
            ));
        }
    }

    ranking
}

impl WinRate {
    pub fn get_winrate(&self) -> f32 {
        self.num_wins as f32 / self.num_matches as f32
    }
}
