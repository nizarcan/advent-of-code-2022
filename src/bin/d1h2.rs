use std::fs;

fn main() {
    let file_name = "d1data.txt";
    println!("reading file {file_name:?}");

    const ELF_COUNT: usize = 3;
    
    let mut total_elf_calorie = 0;
    let mut current_elf_calorie = 0;
    let mut elf_calorie_array: [i32; ELF_COUNT] = [0; ELF_COUNT];

    for line in fs::read_to_string(file_name).unwrap().lines() {
        if line == "" {
            for idx in (0..ELF_COUNT).rev() {
                if idx == ELF_COUNT-1 && elf_calorie_array[idx] < current_elf_calorie {
                    elf_calorie_array[idx] = current_elf_calorie;
                } else if elf_calorie_array[idx] < current_elf_calorie {
                    elf_calorie_array[idx+1] = elf_calorie_array[idx];
                    elf_calorie_array[idx] = current_elf_calorie;
                } else {
                    break
                }
            }
            current_elf_calorie = 0;
        }
        else {
            let line_calorie = line.parse::<i32>().unwrap();
            current_elf_calorie += line_calorie;
        }
    };

    for idx in 0..ELF_COUNT {
        total_elf_calorie += elf_calorie_array[idx];
    }

    println!("elf_calorie_array: {elf_calorie_array:?}");

    println!("max number of calorie an elf carries is {total_elf_calorie:?}");
}


