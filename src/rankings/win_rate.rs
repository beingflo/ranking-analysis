use std::collections::HashMap;

use crate::util::{get_players, Match};

pub struct WinRate {
    num_matches: u32,
    num_wins: u32,
}

pub struct WinRateRanking {
    pub winrates: HashMap<String, WinRate>,
}

pub fn compute_winrate_accuracy(matches: &Vec<Match>) -> f32 {
    let mut num_matches = 0;
    let mut correct_predictions = 0;

    let mut ranking = WinRateRanking {
        winrates: HashMap::new(),
    };

    for p in get_players(&matches) {
        ranking.winrates.insert(
            p.clone(),
            WinRate {
                num_matches: 0,
                num_wins: 0,
            },
        );
    }

    for m in matches.iter() {
        let all_players = m.team_a.iter().chain(m.team_b.iter());
        let (winning_team, losing_team) = if m.team_a_score > m.team_b_score {
            (&m.team_a, &m.team_b)
        } else {
            (&m.team_b, &m.team_a)
        };

        let winrate_winners: f32 = winning_team
            .iter()
            .map(|p| ranking.winrates.get(p).unwrap().get_winrate())
            .sum();
        let winrate_losers: f32 = losing_team
            .iter()
            .map(|p| ranking.winrates.get(p).unwrap().get_winrate())
            .sum();

        num_matches += 1;

        if winrate_winners >= winrate_losers {
            correct_predictions += 1;
        }

        for p in winning_team.iter() {
            ranking.winrates.get_mut(p).unwrap().num_wins += 1;
        }
        for p in all_players {
            ranking.winrates.get_mut(p).unwrap().num_matches += 1;
        }
    }

    correct_predictions as f32 / num_matches as f32
}

impl WinRate {
    pub fn get_winrate(&self) -> f32 {
        if self.num_matches == 0 {
            0.0
        } else {
            self.num_wins as f32 / self.num_matches as f32
        }
    }
}
