pub fn calc(input: &str, iters: i32) -> usize {
    let mut current = String::from(input);
    for _i in 0..iters {
        current = look_and_say(&current);
    }
    current.len()
}

pub fn look_and_say(input: &str) -> String {
    let chars = input.chars().collect::<Vec<char>>();
    let mut new_str = String::new();
    let mut chain_count: usize = 1;
    for (i, c) in input.chars().enumerate() {
        if i == input.len() - 1 {
            new_str.push_str(&chain_count.to_string());
            new_str.push_str(&c.to_string());
            continue;
        }
        if c == chars[i + 1] {
            chain_count += 1;
        } else {
            new_str.push_str(&chain_count.to_string());
            new_str.push_str(&c.to_string());
            chain_count = 1;
        }
    }
    new_str
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_a_sample() {
        assert_eq!(super::calc("1", 5), 6);
    }
    #[test]
    fn part_a() {
        assert_eq!(super::calc("1321131112", 40), 492982);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::calc("1321131112", 50), 6989950);
    }

    #[test]
    fn look_say() {
        assert_eq!(super::look_and_say("21"), "1211");
        assert_eq!(super::look_and_say("111221",), "312211");
    }
}
