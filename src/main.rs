use crate::{
    rankings::{
        trueskill::compute_trueskill_accuracy, weng_lin::compute_weng_lin_accuracy,
        win_rate::compute_winrate_accuracy,
    },
    util::read_matches,
};

pub mod rankings;
pub mod util;

fn main() {
    let matches = read_matches();
    let winrate_accuracy = compute_winrate_accuracy(&matches);
    let weng_lin_accuracy = compute_weng_lin_accuracy(&matches);
    let trueskill_accuracy = compute_trueskill_accuracy(&matches);

    println!("Winrate accuracy: {}", winrate_accuracy);
    println!("Weng Lin accuracy: {}", weng_lin_accuracy);
    println!("TrueSkill accuracy: {}", trueskill_accuracy);
}
