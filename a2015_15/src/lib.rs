use std::{error::Error, str::FromStr};
#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavour: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn parse_val(split: &mut std::str::Split<&str>, n: usize) -> i32 {
        split
            .nth(n)
            .unwrap()
            .strip_suffix(",")
            .unwrap()
            .parse()
            .unwrap()
    }
}

impl FromStr for Ingredient {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");
        Ok(Ingredient {
            capacity: Self::parse_val(&mut split, 2),
            durability: Self::parse_val(&mut split, 1),
            flavour: Self::parse_val(&mut split, 1),
            texture: Self::parse_val(&mut split, 1),
            calories: split.nth(1).unwrap().parse().unwrap(),
        })
    }
}

pub fn part_a_b(input: &str, cap_calories: bool) -> i32 {
    let ingredients: Vec<Ingredient> = input.lines().map(|s| s.parse().unwrap()).collect();
    if ingredients.len() != 4 {
        panic!()
    }

    let mut best_score = 0;

    for i0 in 0..=100 {
        for i1 in 0..=100 - i0 {
            for i2 in 0..=100 - i0 - i1 {
                for i3 in 0..=100 - i0 - i1 - i2 {
                    let calories = 0.max(
                        ingredients[0].calories * i0
                            + ingredients[1].calories * i1
                            + ingredients[2].calories * i2
                            + ingredients[3].calories * i3,
                    );
                    if cap_calories && calories != 500 {
                        continue;
                    }

                    let capacity = 0.max(
                        ingredients[0].capacity * i0
                            + ingredients[1].capacity * i1
                            + ingredients[2].capacity * i2
                            + ingredients[3].capacity * i3,
                    );
                    let durability = 0.max(
                        ingredients[0].durability * i0
                            + ingredients[1].durability * i1
                            + ingredients[2].durability * i2
                            + ingredients[3].durability * i3,
                    );
                    let flavour = 0.max(
                        ingredients[0].flavour * i0
                            + ingredients[1].flavour * i1
                            + ingredients[2].flavour * i2
                            + ingredients[3].flavour * i3,
                    );
                    let texture = 0.max(
                        ingredients[0].texture * i0
                            + ingredients[1].texture * i1
                            + ingredients[2].texture * i2
                            + ingredients[3].texture * i3,
                    );
                    best_score = best_score.max(capacity * durability * flavour * texture);
                }
            }
        }
    }

    best_score
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("input.txt");
    #[test]
    fn part_a() {
        assert_eq!(super::part_a_b(INPUT, false), 13882464);
    }
    #[test]
    fn part_b() {
        assert_eq!(super::part_a_b(INPUT, true), 11171160);
    }
}
