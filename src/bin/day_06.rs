use std::collections::HashMap;
use std::convert::From;

fn main() {
    let system = std::fs::read_to_string("src/inputs/day_06")
        .expect("Failed to read input file")
        .parse::<System>()
        .unwrap();
    let part_1 = system.total_orbits();
    let part_2 = system.transfer_distance(&ObjectID::from("YOU"), &ObjectID::from("SAN"));

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct ObjectID(String);

impl From<String> for ObjectID {
    fn from(s: String) -> Self {
        ObjectID(s)
    }
}

impl From<&str> for ObjectID {
    fn from(s: &str) -> Self {
        ObjectID(s.to_string())
    }
}

struct System {
    orbit_map: HashMap<ObjectID, ObjectID>,
}

impl System {
    fn new() -> System {
        System {
            orbit_map: HashMap::new(),
        }
    }

    fn origin_path(&self, object: &ObjectID) -> Vec<ObjectID> {
        let mut path: Vec<ObjectID> = Vec::new();
        let mut current = object;
        while let Some(next) = self.orbit_map.get(current) {
            path.push((*next).clone());
            current = next;
        }

        path
    }

    fn transfer_distance(&self, a: &ObjectID, b: &ObjectID) -> usize {
        let path_a = self.origin_path(a);
        let path_b = self.origin_path(b);

        let common = path_a
            .iter()
            .rev()
            .zip(path_b.iter().rev())
            .filter(|(c, d)| c == d)
            .count();

        (path_a.len() - 1) + (path_b.len() - 1) - (common - 1) * 2
    }

    fn total_orbits(&self) -> usize {
        self.orbit_map
            .keys()
            .map(|object| self.origin_path(object).len())
            .sum()
    }
}

impl std::str::FromStr for System {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut system = System::new();
        for line in s.lines() {
            let mut ids = line.split(')');
            let parent: ObjectID = ids.next().expect("First ID missing").to_string().into();
            let child: ObjectID = ids.next().expect("Second ID missing").to_string().into();

            match system.orbit_map.insert(child.clone(), parent) {
                Some(_) => {
                    Err(format!("Object {:?} has multiple orbits ({:?}", child, line).to_string())
                }
                None => Ok(()),
            }?;
        }

        Ok(system)
    }
}

#[cfg(test)]
mod day_06 {
    use super::*;

    #[test]
    fn orbit_count() {
        let output = "COM)B\n\
                      B)C\n\
                      C)D\n\
                      D)E\n\
                      E)F\n\
                      B)G\n\
                      G)H\n\
                      D)I\n\
                      E)J\n\
                      J)K\n\
                      K)L"
        .parse::<System>()
        .unwrap()
        .total_orbits();

        assert_eq!(output, 42);
    }

    #[test]
    fn path() {
        let output = "COM)B\n\
                      B)C\n\
                      C)D\n\
                      D)E\n\
                      E)F\n\
                      B)G\n\
                      G)H\n\
                      D)I\n\
                      E)J\n\
                      J)K\n\
                      K)L\n\
                      K)YOU\n\
                      I)SAN"
            .parse::<System>()
            .unwrap()
            .transfer_distance(&ObjectID::from("YOU"), &ObjectID::from("SAN"));

        assert_eq!(output, 4);
    }
}
