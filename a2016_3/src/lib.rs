use std::collections::HashMap;

struct Town {
    current_pos: (i32, i32),
    robot_pos: (i32, i32),
    visited: HashMap<(i32, i32), u32>,
    robot_enabled: bool,
    robots_turn: bool,
}

impl Town {
    fn new(robot_enabled: bool) -> Self {
        Self {
            current_pos: (0, 0),
            robot_pos: (0, 0),
            visited: HashMap::from([((0, 0), if robot_enabled { 2 } else { 1 })]),
            robot_enabled: robot_enabled,
            robots_turn: false,
        }
    }

    fn travel(&mut self, direction: &Direction) -> () {
        let mut target_pos = if self.robots_turn && self.robot_enabled {
            &mut self.robot_pos
        } else {
            &mut self.current_pos
        };

        match direction {
            Direction::North => target_pos.1 += 1,
            Direction::South => target_pos.1 -= 1,
            Direction::East => target_pos.0 += 1,
            Direction::West => target_pos.0 -= 1,
        };
        *self.visited.entry(*target_pos).or_insert(0) += 1;

        if self.robot_enabled {
            self.robots_turn = !self.robots_turn;
        }
    }
}

#[derive(PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn from_char(input: char) -> Self {
        match input {
            '^' => Self::North,
            'v' => Self::South,
            '>' => Self::East,
            '<' => Self::West,
            _ => panic!("Bad char!"),
        }
    }
}

pub fn part_a(input: &str) -> usize {
    let mut town = Town::new(false);
    let directions: Vec<Direction> = input.trim().chars().map(Direction::from_char).collect();
    directions
        .iter()
        .for_each(|direction| town.travel(direction));
    town.visited.len()
}

pub fn part_b(input: &str) -> usize {
    let mut town = Town::new(true);
    let directions: Vec<Direction> = input.trim().chars().map(Direction::from_char).collect();
    directions
        .iter()
        .for_each(|direction| town.travel(direction));
    town.visited.len()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(INPUT), 2572);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(INPUT), 2631);
    }

    #[test]
    fn parse_direction() {
        assert_eq!(super::Direction::from_char('v'), super::Direction::South);
    }
}
