use crate::{rankings::win_rate::compute_ranking, util::read_matches};

pub mod rankings;
pub mod util;

fn main() {
    let matches = read_matches();
    let ranking = compute_ranking(&matches);

    for e in ranking.winrate_history.get("251").unwrap() {
        println!("{{ date: '{}', winrate: {} }},", e.0, e.1);
    }
}
