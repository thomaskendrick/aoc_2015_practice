struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn calc_surface(&self) -> u32 {
        (2 * self.length * self.width)
            + (2 * self.width * self.height)
            + (2 * self.height * self.length)
    }

    fn calc_slack(&self) -> u32 {
        let mut sides = [self.length, self.height, self.width];
        sides.sort();
        return sides[0] * sides[1];
    }

    fn calc_ribbon(&self) -> u32 {
        let mut sides = [self.length, self.height, self.width];
        sides.sort();
        (2 * sides[0]) + (2 * sides[1]) + (self.length * self.width * self.height)
    }
}

fn parse_presents(input: &str) -> Vec<Present> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split('x');
            Present {
                length: parts.next().unwrap().parse().unwrap(),
                width: parts.next().unwrap().parse().unwrap(),
                height: parts.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

pub fn part_a(input: &str) -> u32 {
    parse_presents(input).iter().fold(0, |acc, present| {
        acc + present.calc_surface() + present.calc_slack()
    })
}

pub fn part_b(input: &str) -> u32 {
    parse_presents(input)
        .iter()
        .fold(0, |acc, present| acc + present.calc_ribbon())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(INPUT), 1598415);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(INPUT), 3812909);
    }

    #[test]
    fn calc_surface_test() {
        let present = super::Present {
            length: 2,
            width: 3,
            height: 4,
        };
        assert_eq!(present.calc_surface(), 52)
    }
    #[test]
    fn calc_slack_test() {
        let present = super::Present {
            length: 2,
            width: 3,
            height: 4,
        };
        assert_eq!(present.calc_slack(), 6)
    }
}
