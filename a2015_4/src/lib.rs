use md5;

pub fn part_a(input: &str) -> i64 {
    let mut count = 0;
    loop{
        let mut target_string = String::from(input.trim());
        target_string.push_str(&count.to_string());
        let digest = format!("{:x}", md5::compute(&target_string));
        if digest.starts_with("00000") {break;}
        count += 1;
    }
    count
}
pub fn part_b(input: &str) -> i64 {
    let mut count = 0;
    loop{
        let mut target_string = String::from(input.trim());
        target_string.push_str(&count.to_string());
        let digest = format!("{:x}", md5::compute(&target_string));
        if digest.starts_with("000000") {break;}
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(INPUT), 117946);
    }
    #[test]
    fn part_b() {
        assert_eq!(super::part_b(INPUT), 3938038);
    }
}
