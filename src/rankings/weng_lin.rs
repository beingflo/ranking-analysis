use crate::util::{get_players, Match};
use skillratings::{
    weng_lin::{expected_score_two_teams, weng_lin_two_teams, WengLinConfig, WengLinRating},
    Outcomes,
};
use std::collections::HashMap;

pub type WengLinRanking = HashMap<String, WengLinRating>;

pub fn compute_weng_lin_accuracy(matches: &Vec<Match>) -> f32 {
    let mut num_matches = 0;
    let mut correct_predictions = 0;

    let mut ranking = HashMap::new();

    for p in get_players(&matches) {
        ranking.insert(p.clone(), WengLinRating::new());
    }

    for (idx, m) in matches.iter().enumerate() {
        let (probability_a, _probability_b) = expected_score_two_teams(
            &[
                ranking.get(&m.team_a[0]).unwrap().clone(),
                ranking.get(&m.team_a[1]).unwrap().clone(),
            ],
            &[
                ranking.get(&m.team_b[0]).unwrap().clone(),
                ranking.get(&m.team_b[1]).unwrap().clone(),
            ],
            &WengLinConfig::new(),
        );

        let team_a_wins = m.team_a_score > m.team_b_score;

        if idx > matches.len() - 1000 {
            num_matches += 1;

            if probability_a >= 0.5 && team_a_wins {
                correct_predictions += 1;
            }
            if probability_a <= 0.5 && !team_a_wins {
                correct_predictions += 1;
            }
        }

        let outcome = if team_a_wins {
            Outcomes::WIN
        } else {
            Outcomes::LOSS
        };

        let (new_a, new_b) = weng_lin_two_teams(
            &[
                ranking.get(&m.team_a[0]).unwrap().clone(),
                ranking.get(&m.team_a[1]).unwrap().clone(),
            ],
            &[
                ranking.get(&m.team_b[0]).unwrap().clone(),
                ranking.get(&m.team_b[1]).unwrap().clone(),
            ],
            &outcome,
            &WengLinConfig::new(),
        );

        ranking.insert(m.team_a[0].clone(), new_a[0]);
        ranking.insert(m.team_a[1].clone(), new_a[1]);
        ranking.insert(m.team_b[0].clone(), new_b[0]);
        ranking.insert(m.team_b[1].clone(), new_b[1]);
    }

    correct_predictions as f32 / num_matches as f32
}
