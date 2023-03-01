fn main() {
    let lines = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let highest = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<usize>())
        .max()
        .unwrap();

    println!("{:?}", highest);
    assert!(highest == 72602);
}
