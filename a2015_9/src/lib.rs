use std::collections::{HashMap, HashSet};

struct HeapsAlgo<T> {
    permutations: Vec<Vec<T>>,
}

impl<T: Clone> HeapsAlgo<T> {
    fn new(list: &mut [T]) -> Self {
        let mut ha = Self { permutations: Vec::new() };
        ha.generate(list.len(), list);
        ha
    }
    fn generate(&mut self, size: usize, list: &mut [T]) {
        if size == 1 {
            self.permutations.push(Vec::from(list));
        } else {
            self.generate(size - 1, list);
            for i in 0..size - 1 {
                if size % 2 == 0 {
                    list.swap(i, size - 1)
                } else {
                    list.swap(0, size - 1)
                }
                self.generate(size - 1, list);
            }
        }
    }
}

fn gen_distance_map(s: &str) -> HashMap<(&str, &str), i32> {
    let mut distance_map: HashMap<(&str, &str), i32> = HashMap::new();
    for l in s.lines() {
        let split: Vec<&str> = l.split(' ').collect();
        distance_map.insert((split[0], split[2]), split[4].parse().unwrap());
        distance_map.insert((split[2], split[0]), split[4].parse().unwrap());
    }
    distance_map
}

fn gen_uniq_city(s: &str) -> Vec<&str> {
    let mut city_list: HashSet<&str> = HashSet::new();
    for l in s.lines() {
        let split: Vec<&str> = l.split(' ').collect();
        city_list.insert(split[0]);
        city_list.insert(split[2]);
    }
    city_list.into_iter().collect()
}

fn part_a_b(s: &str) -> (i32, i32) {
    let distance_map = gen_distance_map(s);
    let mut cities = gen_uniq_city(s);
    let ha: HeapsAlgo<&str> = HeapsAlgo::new(&mut cities);
    let mut shortest = i32::MAX;
    let mut longest = 0;
    for route in ha.permutations {
        let distance = route.windows(2).fold(0, |acc, window| {
            acc + *distance_map.get(&(window[0], window[1])).unwrap()
        });

        if distance < shortest {
            shortest = distance;
        }
        if distance > longest {
            longest = distance;
        }
    }
    (shortest, longest)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_a_b_test() {
        assert_eq!(part_a_b(INPUT), (141, 736));
    }
}
