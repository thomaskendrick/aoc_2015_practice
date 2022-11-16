//children: 3
//cats: 7
//samoyeds: 2
//pomeranians: 3
//akitas: 0
//vizslas: 0
//goldfish: 5
//trees: 3
//cars: 2
//perfumes: 1

type Sue = (u32, Vec<SueClue>);

#[derive(Debug)]
enum SueClue {
    Children(u32),
    Cats(u32),
    Samoyeds(u32),
    Pomeranians(u32),
    Akitas(u32),
    Vizlas(u32),
    Goldfish(u32),
    Trees(u32),
    Cars(u32),
    Perfumes(u32),
}

fn clean_num(s: &str) -> u32 {
    s.chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn parse_to_sue(s: &str) -> Sue {
    let mut split = s.split(" ").skip(1);
    let mut sue_clues = Vec::new();
    let sue_number = split
        .next()
        .unwrap()
        .strip_suffix(":")
        .unwrap()
        .parse()
        .unwrap();

    for _ in 0..3 {
        let clue = match split.next().unwrap() {
            "children:" => SueClue::Children(clean_num(split.next().unwrap())),
            "cats:" => SueClue::Cats(clean_num(split.next().unwrap())),
            "samoyeds:" => SueClue::Samoyeds(clean_num(split.next().unwrap())),
            "pomeranians:" => SueClue::Pomeranians(clean_num(split.next().unwrap())),
            "akitas:" => SueClue::Akitas(clean_num(split.next().unwrap())),
            "vizslas:" => SueClue::Vizlas(clean_num(split.next().unwrap())),
            "goldfish:" => SueClue::Goldfish(clean_num(split.next().unwrap())),
            "trees:" => SueClue::Trees(clean_num(split.next().unwrap())),
            "cars:" => SueClue::Cars(clean_num(split.next().unwrap())),
            "perfumes:" => SueClue::Perfumes(clean_num(split.next().unwrap())),
            _ => panic!("Invalid Sue!"),
        };
        sue_clues.push(clue);
    }
    (sue_number, sue_clues)
}

pub fn part_a(input: &str) -> u32 {
    let sues: Vec<_> = input.lines().map(|l| parse_to_sue(l)).collect();
    'sue_loop: for (number, clues) in sues {
        for clue in clues {
            match clue {
                SueClue::Children(x) => {
                    if x != 3 {
                        continue 'sue_loop;
                    }
                }
                SueClue::Cats(x) => {
                    if x != 7 {
                        continue 'sue_loop;
                    }
                }
                SueClue::Samoyeds(x) => {
                    if x != 2 {
                        continue 'sue_loop;
                    }
                }
                SueClue::Pomeranians(x) => {
                    if x != 3 {
                        continue 'sue_loop;
                    }
                }
                SueClue::Akitas(x) => {
                    if x != 0 {
                        continue 'sue_loop;
                    }
                }
                SueClue::Vizlas(x) => {
                    if x != 0 {
                        continue 'sue_loop;
                    }
                }
                SueClue::Goldfish(x) => {
                    if x != 5 {
                        continue 'sue_loop;
                    }
                }
                SueClue::Trees(x) => {
                    if x != 3 {
                        continue 'sue_loop;
                    }
                }
                SueClue::Cars(x) => {
                    if x != 2 {
                        continue 'sue_loop;
                    }
                }
                SueClue::Perfumes(x) => {
                    if x != 1 {
                        continue 'sue_loop;
                    }
                }
            }
        }
        return number;
    }
    1
}
pub fn part_b(input: &str) -> u32 {
    let sues: Vec<_> = input.lines().map(|l| parse_to_sue(l)).collect();
    'sue_loop: for (number, clues) in sues {
        for clue in clues {
            match clue {
                SueClue::Children(x) => {
                    if x != 3 {
                        continue 'sue_loop;
                    }
                }
                SueClue::Cats(x) => {
                    if x <= 7 {
                        continue 'sue_loop;
                    }
                }
                SueClue::Samoyeds(x) => {
                    if x != 2 {
                        continue 'sue_loop;
                    }
                }
                SueClue::Pomeranians(x) => {
                    if x >= 3 {
                        continue 'sue_loop;
                    }
                }
                SueClue::Akitas(x) => {
                    if x != 0 {
                        continue 'sue_loop;
                    }
                }
                SueClue::Vizlas(x) => {
                    if x != 0 {
                        continue 'sue_loop;
                    }
                }
                SueClue::Goldfish(x) => {
                    if x >= 5 {
                        continue 'sue_loop;
                    }
                }
                SueClue::Trees(x) => {
                    if x <= 3 {
                        continue 'sue_loop;
                    }
                }
                SueClue::Cars(x) => {
                    if x != 2 {
                        continue 'sue_loop;
                    }
                }
                SueClue::Perfumes(x) => {
                    if x != 1 {
                        continue 'sue_loop;
                    }
                }
            }
        }
        return number;
    }
    1
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(INPUT), 103);
    }
    #[test]
    fn part_b() {
        assert_eq!(super::part_b(INPUT), 405);
    }
}
