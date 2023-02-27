use std::io;

fn main() {
    let mut calorie_tracker = 0;
    let mut highest_so_far = 0;

    let lines = io::stdin().lines();

    for line in lines {
        let calorie = match line {
            Ok(c) => c,
            Err(_) => {
                println!("Problem reading a line, ignoring...");

                String::from("")
            }
        };

        if calorie.trim().is_empty() {
            if calorie_tracker > highest_so_far {
                highest_so_far = calorie_tracker
            }

            calorie_tracker = 0
        } else {
            let calorie = calorie.parse::<u32>().unwrap();
            calorie_tracker += calorie;
        }
    }

    println!("{:?}", highest_so_far);
    assert!(highest_so_far == 72602);
}
