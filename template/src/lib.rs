pub fn part_a(input: &str) -> i64 {
    for line in input.trim().split('\n') {
    }
    0
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(INPUT), 0);
    }
}
