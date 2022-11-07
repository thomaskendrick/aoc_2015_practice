pub fn part_a(input: &str) -> i64 {
    for line in input.trim().split('\n') {
    }
    0
}

#[cfg(test)]
mod tests {
    const input: &str = include_str1("input.txt");

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(input), 0);
    }
}
