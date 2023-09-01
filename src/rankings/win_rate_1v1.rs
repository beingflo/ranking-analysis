use std::collections::HashMap;

use crate::util::{get_players, Match};

pub struct WinRate {
    num_matches: u32,
    num_wins: u32,
}

pub struct WinRateRanking {
    pub winrates: HashMap<String, WinRate>,
}

pub fn compute_winrate_1v1_accuracy(matches: &Vec<Match>) -> f32 {
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

    for (idx, m) in matches.iter().enumerate() {
        let all_players = m.team_a.iter().chain(m.team_b.iter());
        let (winning_team, _losing_team) = if m.team_a_score > m.team_b_score {
            (&m.team_a, &m.team_b)
        } else {
            (&m.team_b, &m.team_a)
        };
        let team_a_wins = m.team_a_score > m.team_b_score;

        let winrate_a0 = ranking.winrates.get(&m.team_a[0]).unwrap().get_winrate();
        let winrate_a1 = ranking.winrates.get(&m.team_a[1]).unwrap().get_winrate();
        let winrate_b0 = ranking.winrates.get(&m.team_b[0]).unwrap().get_winrate();
        let winrate_b1 = ranking.winrates.get(&m.team_b[1]).unwrap().get_winrate();

        let expected_win1 = winrate_a0 - winrate_b1;
        let expected_win2 = winrate_a1 - winrate_b0;

        if idx > matches.len() - 1000 {
            num_matches += 1;

            if expected_win1 + expected_win2 >= 0.0 && team_a_wins {
                correct_predictions += 1;
            }
            if expected_win1 + expected_win2 <= 0.0 && !team_a_wins {
                correct_predictions += 1;
            }
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
