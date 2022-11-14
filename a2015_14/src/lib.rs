use core::str::FromStr;
use std::{collections::HashMap, error::Error};

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

impl FromStr for Reindeer {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spl = s.split(" ");
        Ok(Reindeer {
            name: String::from(spl.next().unwrap()),
            speed: spl.nth(2).unwrap().parse()?,
            fly_time: spl.nth(2).unwrap().parse()?,
            rest_time: spl.nth(6).unwrap().parse()?,
        })
    }
}

impl Reindeer {
    fn calc_distance(&self, time: u32) -> u32 {
        let mut time_elapsed = 0;
        let mut distance_travelled = 0;
        while time_elapsed < time {
            // Flying
            if self.fly_time <= time - time_elapsed {
                time_elapsed += self.fly_time;
                distance_travelled += self.speed * self.fly_time;
            } else {
                distance_travelled += self.speed * (time - time_elapsed);
                time_elapsed = time;
            }

            // Resting
            if time_elapsed != time {
                time_elapsed += self.rest_time;
            }
        }
        distance_travelled
    }
}

pub fn part_a(input: &str, time: u32) -> u32 {
    let reindeers: Vec<Reindeer> = input.lines().map(|l| l.parse().unwrap()).collect();

    let mut winner = 0;
    for reindeer in reindeers {
        winner = winner.max(reindeer.calc_distance(time));
    }
    winner
}
pub fn part_b(input: &str, time: u32) -> u32 {
    let reindeers: Vec<Reindeer> = input.lines().map(|l| l.parse().unwrap()).collect();

    let mut scoreboard: HashMap<String, u32> =
        reindeers.iter().map(|r| (r.name.to_owned(), 0)).collect();

    for t in 1..=time {
        let mut leader_distance = 0;
        let mut leaders: Vec<String> = Vec::new();

        for reindeer in &reindeers {
            let distance = reindeer.calc_distance(t);
            if distance > leader_distance {
                leader_distance = distance;
                leaders.clear();
                leaders.push(reindeer.name.to_owned());
            } else if distance == leader_distance {
                leaders.push(reindeer.name.to_owned())
            }
        }
        for leader in leaders {
            *scoreboard.get_mut(&leader).unwrap() += 1;
        }
    }
    scoreboard.iter().map(|(_, score)| *score).max().unwrap()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("input.txt");
    use super::*;

    #[test]
    fn part_a_test() {
        assert_eq!(part_a(INPUT, 2503), 2640);
    }
    #[test]
    fn part_b_test() {
        assert_eq!(part_b(INPUT, 2503), 1102);
    }
    #[test]
    fn calc_disance_test_comet() {
        let comet = Reindeer {
            name: String::from("Comet"),
            speed: 14,
            fly_time: 10,
            rest_time: 127,
        };
        assert_eq!(comet.calc_distance(1000), 1120);
    }
    #[test]
    fn calc_disance_test_dancer() {
        let dancer = Reindeer {
            name: String::from("Dancer"),
            speed: 16,
            fly_time: 11,
            rest_time: 162,
        };
        assert_eq!(dancer.calc_distance(1000), 1056);
    }
}
