use std::{fs, collections::HashMap};

fn main() {
    let file_name = "d2data.txt";
    println!("reading file {file_name:?}");

    let game_map = HashMap::from([
        ("A X", "C"),
        ("B X", "A"),
        ("C X", "B"),
        ("A Y", "A"),
        ("B Y", "B"),
        ("C Y", "C"),
        ("A Z", "B"),
        ("B Z", "C"),
        ("C Z", "A"),
    ]);

    let result_points = HashMap::from([
        ("X", 0),
        ("Y", 3),
        ("Z", 6),
    ]);

    let play_values = HashMap::from([
        ("A", 1),
        ("B", 2),
        ("C", 3),
    ]);

    let mut score = 0;

    for line in fs::read_to_string(file_name).unwrap().lines() {
        let game_result = line.split_whitespace().nth(1).unwrap_or_default();
        let game_point = match result_points.get(game_result) {
            Some(value) => *value,
            None => 0,
        };

        let played_hand = match game_map.get(line) {
            Some(value) => *value,
            None => "X",
        };
        let played_hand_point = match play_values.get(played_hand) {
            Some(value) => *value,
            None => 0,
        };

        score += game_point + played_hand_point;
    }

    println!("Score: {score:?}");
}
