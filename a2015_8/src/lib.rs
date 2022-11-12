fn check_string(s: &str) -> usize {
    let chars = s.chars().collect::<Vec<char>>();
    let mut i = 1;
    let mut escaped: usize = 0;

    while i < chars.len() - 1 {
        escaped += 1;
        if chars[i] == '\\' {
            if chars[i + 1] == 'x' {
                i += 3;
            } else {
                i += 1;
            }
        }
        i += 1;
    }
    s.len() - escaped

}

fn encode_string(s: &str) -> usize {
    let extra_escape = format!("{}", s.escape_default());
    (extra_escape.len() + 2) - s.len()
}

pub fn part_a(s: &str) -> usize {
    s.lines().map(check_string).sum()
}

pub fn part_b(s: &str) -> usize {
    s.lines().map(encode_string).sum()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("input.txt");
    const SAMPLE: &str = include_str!("sample.txt");
    use super::*;

    #[test]
    fn part_a_test() {
        assert_eq!(part_a(INPUT), 1350);
    }
    #[test]
    fn part_b_test() {
        assert_eq!(part_b(INPUT), 2085);
    }

    #[test]
    fn check_string_test() {
        let mut iter = SAMPLE.lines();
        assert_eq!(check_string(iter.next().unwrap()), 2);
        assert_eq!(check_string(iter.next().unwrap()), 2);
        assert_eq!(check_string(iter.next().unwrap()), 3);
        assert_eq!(check_string(iter.next().unwrap()), 5);
    }
    #[test]
    fn encode_string_test() {
        let mut iter = SAMPLE.lines();
        assert_eq!(encode_string(iter.next().unwrap()), 4);
        assert_eq!(encode_string(iter.next().unwrap()), 4);
        assert_eq!(encode_string(iter.next().unwrap()), 6);
        assert_eq!(encode_string(iter.next().unwrap()), 5);
    }
}
