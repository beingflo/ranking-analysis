use crate::{rankings::win_rate::compute_winrate_accuracy, util::read_matches};

pub mod rankings;
pub mod util;

fn main() {
    let matches = read_matches();
    let winrate_accuracy = compute_winrate_accuracy(&matches);

    println!("Winrate accuracy: {}", winrate_accuracy);
}
