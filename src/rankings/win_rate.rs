use std::collections::HashMap;

use crate::util::{get_players, Match};

pub struct WinRate {
    num_matches: u32,
    num_wins: u32,
}

pub struct WinRateRanking {
    pub winrates: HashMap<String, WinRate>,
}

pub fn compute_ranking(matches: &Vec<Match>) -> WinRateRanking {
    let mut ranking = WinRateRanking {
        winrates: HashMap::new(),
    };

    for p in get_players(&matches) {
        ranking.winrates.insert(
            p,
            WinRate {
                num_matches: 0,
                num_wins: 0,
            },
        );
    }

    for m in matches.iter() {
        let [winning_team, losing_team] = if m.team_a_score > m.team_b_score {
            [&m.team_a, &m.team_b]
        } else {
            [&m.team_b, &m.team_a]
        };

        for p in winning_team.iter() {
            ranking.winrates.get_mut(p).unwrap().num_matches += 1;
            ranking.winrates.get_mut(p).unwrap().num_wins += 1;
        }
        for p in losing_team.iter() {
            ranking.winrates.get_mut(p).unwrap().num_matches += 1;
        }
    }

    ranking
}

impl WinRate {
    pub fn get_winrate(&self) -> f32 {
        self.num_wins as f32 / self.num_matches as f32
    }
}
