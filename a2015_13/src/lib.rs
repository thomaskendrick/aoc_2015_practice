use std::collections::HashMap;
use util::heaps::HeapsAlgo;

type OpinionMap = HashMap<String, i32>;
type PeopleMap = HashMap<String, OpinionMap>;

pub fn part_a_b(input: &str, include_me: bool) -> i32 {
    let mut people: PeopleMap = HashMap::new();

    for l in input.lines() {
        let mut spl = l.split(" ");
        let person = spl.next().unwrap();
        let modifier;
        spl.next();
        if let "lose" = spl.next().unwrap() {
            modifier = -1
        } else {
            modifier = 1
        }
        let val: i32 = spl.next().unwrap().parse().unwrap();
        let opinion_person = spl.nth(6).unwrap().strip_suffix(".").unwrap();

        if people.contains_key(person) {
            let opinion_map = people.get_mut(person).unwrap();
            opinion_map.insert(String::from(opinion_person), val * modifier);
        } else {
            let opinion_map: OpinionMap;
            if include_me {
                opinion_map = HashMap::from([
                    (String::from(opinion_person), val * modifier),
                    (String::from("Tom"), 0),
                ]);
            } else {
                opinion_map = HashMap::from([(String::from(opinion_person), val * modifier)]);
            }
            people.insert(String::from(person), opinion_map);
        }
    }

    let mut people_names: Vec<String> = people.keys().cloned().collect();
    if include_me {
        people_names.push(String::from("Tom"));
    }
    let ha = HeapsAlgo::new(&people_names);

    let mut optimal_happyness: i32 = i32::MIN;

    for p in ha.permutations {
        let mut tot_happiness: i32 = 0;
        for (i, person) in p.iter().enumerate() {
            if person == "Tom" {
                continue;
            };
            let mut person_happiness_mod = 0;
            if i == 0 {
                person_happiness_mod += people.get(person).unwrap().get(&p[p.len() - 1]).unwrap();
            } else {
                person_happiness_mod += people.get(person).unwrap().get(&p[i - 1]).unwrap();
            }
            if i == p.len() - 1 {
                person_happiness_mod += people.get(person).unwrap().get(&p[0]).unwrap();
            } else {
                person_happiness_mod += people.get(person).unwrap().get(&p[i + 1]).unwrap();
            }
            tot_happiness += person_happiness_mod;
        }
        optimal_happyness = optimal_happyness.max(tot_happiness);
    }
    optimal_happyness
}

#[cfg(test)]
mod tests {
    const SAMPLE: &str = include_str!("sample.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_a_sample() {
        assert_eq!(super::part_a_b(SAMPLE, false), 330);
    }
    #[test]
    fn part_a_b() {
        assert_eq!(super::part_a_b(INPUT, false), 733);
    }
    #[test]
    fn part_a_b_with_me() {
        assert_eq!(super::part_a_b(INPUT, true), 725);
    }
}
