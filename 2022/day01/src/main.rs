use std::fs;

fn main() {
    let file_path = "./src/input.txt";
    println!("reading input data from {}", file_path);

    let mut most_carbs: i32 = 0;

    let raw_input = fs::read_to_string(file_path).expect("error reading file");
    let elves = raw_input.split("\n\n");
    for elf in elves {
        let bag = elf.split("\n");
        let mut carbs_sum: i32 = 0;
        for carbs in bag {
            let a: i32 = carbs.parse().unwrap();
            carbs_sum += a;
        }
        if carbs_sum > most_carbs {
            most_carbs = carbs_sum;
        }
    }
    println!("highest carb held: {}", most_carbs);
}
