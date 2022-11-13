pub fn part_a(input: &str) -> isize {
    let mut number_list: Vec<isize> = Vec::new();
    let mut number_chars: Vec<char> = Vec::new();
    for c in input.chars() {
        if c.is_numeric() || c == '-' {
            number_chars.push(c);
        } else if !number_chars.is_empty() {
            number_list.push(number_chars.iter().collect::<String>().parse().unwrap());
            number_chars.clear();
        }
    }
    number_list.iter().sum()
}

pub fn part_b(input: &str) -> isize {
    let mut number_list: Vec<(isize, usize)> = Vec::new();
    let mut number_chars: Vec<char> = Vec::new();

    let mut opening_bracket_locs: Vec<usize> = Vec::new();
    let mut red_opening_bracket_locs: Vec<usize> = Vec::new();
    let mut bad_range: Vec<(usize, usize)> = Vec::new();

    let mut red_ignores: usize = 0;

    let input_chars: Vec<char> = input.chars().collect();

    for (i, c) in input_chars.iter().enumerate() {
        if *c == '{' && red_ignores == 0 {
            opening_bracket_locs.push(i);
        }
        if red_ignores == 0 && *c == '}' {
            opening_bracket_locs.pop();
        }

        if i < input_chars.len() - 4
            && input_chars[i..=i + 4] == [':', '"', 'r', 'e', 'd']
            && red_ignores == 0
        {
            red_ignores += 1;
            red_opening_bracket_locs.push(opening_bracket_locs.pop().unwrap());
        }

        if red_ignores >= 1 {
            if *c == '{' {
                red_ignores += 1;
            }
            if *c == '}' {
                red_ignores -= 1;
                if red_ignores == 0 {
                    bad_range.push((red_opening_bracket_locs.pop().unwrap(), i));
                }
            }
        }

        if c.is_numeric() || *c == '-' {
            number_chars.push(*c);
        } else if !number_chars.is_empty() {
            number_list.push((
                number_chars.iter().collect::<String>().parse().unwrap(),
                i - 1,
            ));
            number_chars.clear();
        }
    }
    number_list.iter().fold(0, |acc, (num, occured_i)| {
        // Check to see if the number occurred in a bad range
        let is_invalid = bad_range
            .iter()
            .any(|(start, end)| start < occured_i && occured_i < end);
        if is_invalid {
            return acc;
        }
        return acc + num;
    })
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("input.txt");
    const TEST_CASE: &str = include_str!("test_case.txt");

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(INPUT), 111754);
    }
    #[test]
    fn part_b() {
        assert_eq!(super::part_b(INPUT), 65402);
    }

    #[test]
    fn part_b_test() {
        assert_eq!(super::part_b(TEST_CASE), 0);
    }

    #[test]
    fn part_b_sample_1() {
        assert_eq!(super::part_b("[1,2,3]"), 6);
    }
    #[test]
    fn part_b_sample_2() {
        assert_eq!(super::part_b("[1,{\"c\":\"red\",\"b\":2},3]"), 4);
    }
    #[test]
    fn part_b_sample_3() {
        assert_eq!(super::part_b("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"), 0);
    }
    #[test]
    fn part_b_sample_4() {
        assert_eq!(super::part_b("[1,\"red\",5]"), 6);
    }
    #[test]
    fn part_b_cust_test_1() {
        assert_eq!(super::part_b("{\"a\":1,\"b\":4,\"c\":[2,2],\"z\":{\"a\":\"red\",\"b\":5,\"c\":{\"a\":\"red\",\"b\":[1,2,3]}}}"), 9);
    }
}
