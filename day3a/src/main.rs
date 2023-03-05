use crate::item::Item;
use std::collections::HashSet;

mod item;

fn main() {
    let mut total_priorities = 0;
    let supplies = include_str!("../input.txt").lines();

    for rucksack in supplies {
        let (compartment_1, compartment_2) = rucksack.split_at(rucksack.len() / 2);

        let compartment_1 = compartment_1
            .bytes()
            .map(Item::try_from)
            .collect::<Result<HashSet<_>, _>>()
            .expect("a lower or uppercase letter");

        let compartment_2 = compartment_2
            .bytes()
            .map(Item::try_from)
            .collect::<Result<HashSet<_>, _>>()
            .expect("a lower or uppercase letter");

        total_priorities += compartment_1
            .intersection(&compartment_2)
            .map(|i| i.priority())
            .last()
            .unwrap();
    }

    println!("{total_priorities}");
    assert!(total_priorities == 7785);
}
