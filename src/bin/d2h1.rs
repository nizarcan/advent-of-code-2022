use std::{fs, collections::HashMap};

fn main() {
    let file_name = "d2data.txt";
    println!("reading file {file_name:?}");

    let game_map = HashMap::from([
        ("A X", 3),
        ("A Y", 6),
        ("A Z", 0),
        ("B X", 0),
        ("B Y", 3),
        ("B Z", 6),
        ("C X", 6),
        ("C Y", 0),
        ("C Z", 3),
    ]);

    let play_values = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);

    let mut score = 0;

    for line in fs::read_to_string(file_name).unwrap().lines() {
        let game_result = match game_map.get(line) {
            Some(value) => *value,
            None => 0,
        };
        
        let played_hand = line.split_whitespace().nth(1).unwrap_or_default();
        if let Some(play_value) = play_values.get(played_hand) {
            score += game_result + *play_value;
        } else {
            println!("Player hand not found: {played_hand:?}");
        }
    }

    println!("Score: {score:?}");
}
