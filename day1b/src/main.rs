fn main() {
    let mut calories: Vec<u32> = Vec::new();
    let mut current_elf = 0;

    let lines = include_str!("../input.txt").lines();

    for calorie in lines {
        if calorie.trim().is_empty() {
            current_elf += 1;
            calories.push(0);
        } else {
            calories[current_elf - 1] += calorie.parse::<u32>().unwrap();
        }
    }

    calories.sort();
    calories.reverse();

    let top_3 = calories[0] + calories[1] + calories[2];

    println!("{}", top_3);
    assert!(top_3 == 207410);
}
