use crate::util::read_matches;

pub mod util;

fn main() {
    let matches = read_matches();
    let filtered = matches
        .iter()
        .filter(|m| m.team_a.contains(&"251".to_string()) || m.team_b.contains(&"251".to_string()))
        .count();

    println!("{}", filtered);
}
