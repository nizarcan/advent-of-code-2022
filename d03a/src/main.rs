use std::{fs, collections::HashMap};


fn get_priority(c: char) -> usize {
    if c.is_ascii_lowercase() {
        return c as usize - 'a' as usize + 1;
    } else {
        return c as usize - 'A' as usize + 27;
    }
}

pub fn main() {
    let file_name = "../data/d03.txt";
    println!("reading file {file_name:?}");

    let mut priorities = HashMap::new();

    for line in fs::read_to_string(file_name).unwrap().lines() {
        let (first_half, second_half) = line.split_at(line.len() / 2);

        for c in first_half.chars().filter(|&x| second_half.contains(x)) {
            let priority = get_priority(c);
            priorities.entry(c).and_modify(|x| *x += priority).or_insert(priority);
            break;
        }
    }

    let total: usize = priorities.values().sum();
    println!("total is: {total:?}");
}
