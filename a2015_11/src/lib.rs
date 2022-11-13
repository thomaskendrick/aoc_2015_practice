use std::char::from_u32;
use std::convert::TryInto;

struct PasswordGenerator {
    current: [char; 8],
}

impl PasswordGenerator {
    fn new(s: &str) -> Self {
        let chars: Vec<char> = s.chars().collect();
        Self {
            current: chars.try_into().unwrap(),
        }
    }
}

impl Iterator for PasswordGenerator {
    type Item = [char; 8];
    fn next(&mut self) -> Option<Self::Item> {
        let mut target = 7;
        loop {
            let t = self.current[target];
            if t == 'z' {
                self.current[target] = 'a';
                if target == 0 {
                    target = 7
                } else {
                    target -= 1;
                }
            } else {
                self.current[target] = from_u32(t as u32 + 1).unwrap();
                break;
            }
        }
        Some(self.current)
    }
}

fn contains_rising_straight(pw: [char; 8]) -> bool {
    pw.windows(3).any(|window| {
        let first_c_num = window[0] as u32;
        window[1] == from_u32(first_c_num + 1).unwrap()
            && window[2] == from_u32(first_c_num + 2).unwrap()
    })
}

fn contains_bad_chars(pw: [char; 8]) -> bool {
    pw.contains(&'i') || pw.contains(&'o') || pw.contains(&'l')
}

fn contains_two_non_overlapping_pairs(pw: [char; 8]) -> bool {
    let mut found_chars: Vec<char> = Vec::new();
    for w in pw.windows(2) {
        if found_chars.contains(&w[0]) {
            continue;
        }
        if w[0] == w[1] {
            found_chars.push(w[0]);
        }
    }
    found_chars.len() > 1
}

fn is_valid_password(pw: [char; 8]) -> bool {
    !contains_bad_chars(pw)
        && contains_two_non_overlapping_pairs(pw)
        && contains_rising_straight(pw)
}

pub fn part_a_b(input: &str) -> (String, String) {
    let mut pw_generator = PasswordGenerator::new(input);
    let one: String = pw_generator
        .find(|pw| is_valid_password(*pw))
        .unwrap()
        .iter()
        .collect();
    let two: String = pw_generator
        .find(|pw| is_valid_password(*pw))
        .unwrap()
        .iter()
        .collect();
    (one, two)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn can_increment() {
        let mut pw = PasswordGenerator::new("aaaaaaay");
        assert_eq!(pw.current, ['a', 'a', 'a', 'a', 'a', 'a', 'a', 'y']);
        assert_eq!(pw.next().unwrap(), ['a', 'a', 'a', 'a', 'a', 'a', 'a', 'z']);
        assert_eq!(pw.next().unwrap(), ['a', 'a', 'a', 'a', 'a', 'a', 'b', 'a']);
    }

    #[test]
    fn part_a_b_test() {
        assert_eq!(
            part_a_b("vzbxkghb"),
            (String::from("vzbxxyzz"), String::from("vzcaabcc"))
        );
    }

    #[test]
    fn contains_rising_test() {
        assert!(!contains_rising_straight([
            'a', 'a', 'a', 'a', 'a', 'a', 'b', 'a'
        ]));
        assert!(!contains_rising_straight([
            'a', 'a', 'a', 'a', 'a', 'a', 'b', 'b'
        ]));
        assert!(contains_rising_straight([
            'a', 'a', 'a', 'a', 'a', 'a', 'b', 'c'
        ]));
        assert!(!contains_rising_straight([
            'a', 'a', 'a', 'x', 'y', 'x', 'a', 'a'
        ]));
        assert!(!contains_rising_straight([
            'a', 'a', 'a', 'x', 'y', 'y', 'a', 'a'
        ]));
        assert!(contains_rising_straight([
            'a', 'a', 'a', 'x', 'y', 'z', 'a', 'a'
        ]));
    }

    #[test]
    fn contains_bad_chars_test() {
        assert!(!contains_bad_chars([
            'a', 'a', 'a', 'a', 'a', 'a', 'b', 'a'
        ]));
        assert!(contains_bad_chars(['a', 'i', 'a', 'a', 'a', 'a', 'b', 'b']));
    }
    #[test]
    fn contains_two_non_overlapping_pairs_test() {
        assert!(!contains_two_non_overlapping_pairs([
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'
        ]));
        assert!(contains_two_non_overlapping_pairs([
            'a', 'a', 'y', 'a', 'a', 'b', 'b', 'b'
        ]));
        assert!(contains_two_non_overlapping_pairs([
            'a', 'b', 'b', 'a', 'b', 'b', 'a', 'a'
        ]));
    }
    #[test]
    fn valid_password_tests() {
        let pw1 = PasswordGenerator::new("hijklmmn");
        assert!(!is_valid_password(pw1.current));
        let pw2 = PasswordGenerator::new("abbceffg");
        assert!(!is_valid_password(pw2.current));
        let pw3 = PasswordGenerator::new("abbcegjk");
        assert!(!is_valid_password(pw3.current));
        let pw4 = PasswordGenerator::new("abcdffaa");
        assert!(is_valid_password(pw4.current));
        let pw5 = PasswordGenerator::new("ghjaabcc");
        assert!(is_valid_password(pw5.current));
    }
}
