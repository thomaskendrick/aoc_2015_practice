use std::convert::TryFrom;

pub fn part_a(input: &str) -> isize {
   let mut floor = 0; 

    for c in input.trim().chars(){
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("unexpected character")
        }
    }
    floor
}

pub fn part_b(input: &str) -> usize {
   let mut floor = 0; 
    for (i, c) in input.trim().chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("unexpected character")
        }

        if floor == -1 { return 1 + i }
    }
    0
}

#[cfg(test)]
mod tests {
    const input: &str = include_str!("input.txt");

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(input), 74);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(input), 1795);
    }
}
