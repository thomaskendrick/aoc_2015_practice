#[derive(PartialEq, Debug)]
enum Op {
    On,
    Off,
    Toggle,
}

type Point = (usize, usize);

#[derive(PartialEq, Debug)]
struct Instruction {
    op: Op,
    start: Point,
    end: Point,
}

#[derive(PartialEq, Debug)]
struct LightGrid {
    map: [[bool; 1000]; 1000],
}

impl LightGrid {
    fn new() -> Self {
        LightGrid {
            map: [[false; 1000]; 1000],
        }
    }
    fn operate_light(&mut self, op: &Op, (x, y): &Point) {
        match op {
            Op::On => self.map[*x][*y] = true,
            Op::Off => self.map[*x][*y] = false,
            Op::Toggle => self.map[*x][*y] = !self.map[*x][*y],
        }
    }
    fn execute(&mut self, ins: Instruction) {
        for x in ins.start.0..=ins.end.0 {
            for y in ins.start.1..=ins.end.1 {
                self.operate_light(&ins.op, &(x, y))
            }
        }
    }
    fn get_lit_count(&self) -> usize {
        self.map.iter().flatten().filter(|light| **light).count()
    }
}

struct LightGridV2 {
    map: [[u8; 1000]; 1000],
}

impl LightGridV2 {
    fn new() -> Self {
        LightGridV2 {
            map: [[0; 1000]; 1000],
        }
    }
    fn operate_light(&mut self, op: &Op, (x, y): &Point) {
        match op {
            Op::On => self.map[*x][*y] += 1,
            Op::Off => {
                if self.map[*x][*y] != 0 {
                    self.map[*x][*y] -= 1
                }
            },
            Op::Toggle => self.map[*x][*y] += 2,
        }
    }
    fn execute(&mut self, ins: Instruction) {
        for x in ins.start.0..=ins.end.0 {
            for y in ins.start.1..=ins.end.1 {
                self.operate_light(&ins.op, &(x, y))
            }
        }
    }
    fn get_brightness_count(&self) -> usize {
        self.map.iter().flatten().fold(0, |acc, v| usize::from(*v) + acc)
    }
}

fn parse_point(s: &str) -> Point {
    let mut split = s.split(",");
    (
        split.next().unwrap().parse().unwrap(),
        split.next().unwrap().parse().unwrap(),
    )
}

impl Instruction {
    fn new(s: &str) -> Self {
        let mut split = s.split(" ");
        let op: Op;

        match split.next() {
            Some(i) => match i {
                "turn" => match split.next() {
                    Some("on") => op = Op::On,
                    Some("off") => op = Op::Off,
                    _ => panic!(),
                },
                "toggle" => op = Op::Toggle,
                _ => panic!(),
            },
            _ => panic!(),
        }
        Instruction {
            op,
            start: parse_point(split.next().unwrap()),
            end: parse_point(split.skip(1).next().unwrap()),
        }
    }
}

pub fn part_a(input: &str) -> usize {
    let instructions: Vec<_> = input.trim().lines().map(|l| Instruction::new(l)).collect();
    let mut light_grid = LightGrid::new();
    instructions
        .into_iter()
        .for_each(|ins| light_grid.execute(ins));
    light_grid.get_lit_count()
}
pub fn part_b(input: &str) -> usize {
    let instructions: Vec<_> = input.trim().lines().map(|l| Instruction::new(l)).collect();
    let mut light_grid = LightGridV2::new();
    instructions
        .into_iter()
        .for_each(|ins| light_grid.execute(ins));
    light_grid.get_brightness_count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(INPUT), 400410);
    }
    #[test]
    fn part_b() {
        assert_eq!(super::part_b(INPUT), 15343601);
    }

    #[test]
    fn parse_point_test() {
        assert_eq!(parse_point("123,23"), (123, 23));
    }

    #[test]
    fn str_to_instruction() {
        let test_instruction = Instruction {
            op: Op::Off,
            start: (199, 133),
            end: (461, 193),
        };
        assert_eq!(
            Instruction::new("turn off 199,133 through 461,193"),
            test_instruction
        );
    }
    #[test]
    fn str_to_instruction_2() {
        let test_instruction = Instruction {
            op: Op::Toggle,
            start: (100, 200),
            end: (300, 400),
        };
        assert_eq!(
            Instruction::new("toggle 100,200 through 300,400"),
            test_instruction
        );
    }

    #[test]
    fn operate_light_test() {
        let mut test_grid = LightGrid::new();
        test_grid.operate_light(&Op::On, &(0, 0));
        test_grid.operate_light(&Op::On, &(1, 0));
        test_grid.operate_light(&Op::Off, &(1, 0));
        assert_eq!(test_grid.map[0][0], true);
        assert_eq!(test_grid.map[1][0], false);
        test_grid.operate_light(&Op::Toggle, &(0, 0));
        test_grid.operate_light(&Op::Toggle, &(1, 0));
        assert_eq!(test_grid.map[0][0], false);
        assert_eq!(test_grid.map[1][0], true);
    }
    #[test]
    fn execute_test() {
        let mut test_grid = LightGrid::new();
        let ins = Instruction {
            op: Op::On,
            start: (0, 0),
            end: (3, 3),
        };
        test_grid.execute(ins);
        assert_eq!(test_grid.map[0][0], true);
        assert_eq!(test_grid.map[1][1], true);
        assert_eq!(test_grid.map[2][2], true);
        assert_eq!(test_grid.map[3][3], true);
        assert_eq!(test_grid.map[4][4], false);
        assert_eq!(test_grid.get_lit_count(), 16)
    }
}
