use std::fs;

fn main() {
    let file_name = "../data/d01.txt";
    println!("reading file {file_name:?}");
    
    let mut max_elf_calorie = 0;
    let mut current_elf_calorie = 0;

    for line in fs::read_to_string(file_name).unwrap().lines() {
        if line == "" {
            max_elf_calorie = std::cmp::max(max_elf_calorie, current_elf_calorie);
            current_elf_calorie = 0;
        }
        else {
            let line_calorie = line.parse::<i32>().unwrap();
            current_elf_calorie += line_calorie;
        }
    };

    println!("max number of calorie an elf carries is {max_elf_calorie:?}");
}

