use crate::{rankings::win_rate::compute_ranking, util::read_matches};

pub mod rankings;
pub mod util;

fn main() {
    let matches = read_matches();
    let ranking = compute_ranking(&matches);

    println!("{}", ranking.winrates.get("251").unwrap().get_winrate());
}
