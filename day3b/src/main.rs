use item::Item;
use std::collections::HashSet;

mod item;

fn main() {
    let rucksacks = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.bytes()
                .map(Item::try_from)
                .collect::<Result<HashSet<_>, _>>()
                .expect("lower or upper case letter")
        })
        .collect::<Vec<_>>();

    let mut total_priority = 0;

    for group in rucksacks.chunks(3) {
        let common = group
            .iter()
            .cloned()
            .reduce(|acc, e| acc.intersection(&e).cloned().collect::<HashSet<_>>())
            .unwrap();

        total_priority += common.iter().map(|i| i.priority()).last().unwrap();
    }

    println!("{total_priority}");
    assert!(total_priority == 2633);
}
