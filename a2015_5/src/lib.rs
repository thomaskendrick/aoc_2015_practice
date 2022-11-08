const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const BAD_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

struct ElfString {
    string: String,
}

impl ElfString {
    fn new(string: &str) -> Self {
        ElfString {
            string: string.to_owned(),
        }
    }

    fn contains_three_vowels(&self) -> bool {
        self.string.chars().filter(|c| VOWELS.contains(c)).count() >= 3
    }

    fn contains_double_letter(&self) -> bool {
        self.string
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .any(|window| match window {
                [a, b] => a == b,
                _ => panic!("bad window"),
            })
    }

    fn contains_bad_strings(&self) -> bool {
        BAD_STRINGS.iter().any(|bs| self.string.contains(bs))
    }

    fn is_nice(&self) -> bool {
        self.contains_three_vowels()
            && self.contains_double_letter()
            && !self.contains_bad_strings()
    }
}

pub fn part_a(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|l| ElfString::new(l))
        .collect::<Vec<ElfString>>()
        .iter()
        .filter(|es| es.is_nice())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(INPUT), 255);
    }

    #[test]
    fn contains_three_vowels_false() {
        let test_elf_string = ElfString::new("foo");
        assert_eq!(test_elf_string.contains_three_vowels(), false);
    }
    #[test]
    fn contains_three_vowels_true() {
        let test_elf_string = ElfString::new("fooa");
        assert_eq!(test_elf_string.contains_three_vowels(), true);
    }
    #[test]
    fn contains_double_letter_true() {
        let test_elf_string = ElfString::new("fooa");
        assert_eq!(test_elf_string.contains_double_letter(), true);
    }
    #[test]
    fn contains_double_letter_false() {
        let test_elf_string = ElfString::new("foa");
        assert_eq!(test_elf_string.contains_double_letter(), false);
    }
    #[test]
    fn contains_bad_string_false() {
        let test_elf_string = ElfString::new("foa");
        assert_eq!(test_elf_string.contains_bad_strings(), false);
    }
    #[test]
    fn contains_bad_string_true() {
        let test_elf_string = ElfString::new("foxya");
        assert_eq!(test_elf_string.contains_bad_strings(), true);
    }

    #[test]
    fn is_nice() {
        let t1 = ElfString::new("ugknbfddgicrmopn");
        assert_eq!(t1.is_nice(), true);

        let t2 = ElfString::new("aaa");
        assert_eq!(t2.is_nice(), true);

        let t3 = ElfString::new("jchzalrnumimnmhp");
        assert_eq!(t3.is_nice(), false);

        let t4 = ElfString::new("haegwjzuvuyypxyu");
        assert_eq!(t4.is_nice(), false);

        let t5 = ElfString::new("dvszwmarrgswjxmb");
        assert_eq!(t5.is_nice(), false);
    }
}
