use std::{fs::File, io::{BufRead, BufReader}, str::FromStr};

const DIAL_SIZE: u64 = 100;

enum Direction {
    Up,
    Down,
}

struct Rotation {
    direction: Direction,
    rotations: u64,
    full_rotations: u64,
}

impl Rotation {
    pub fn rotate(&mut self, position: u64) -> u64 {
        match self.direction {
            Direction::Up => {
                let total_position = position + self.rotations;
                let new_pos = total_position % DIAL_SIZE;

                if total_position > DIAL_SIZE {
                    self.full_rotations += total_position / DIAL_SIZE;
                }
                
                new_pos
            },
            Direction::Down => {
                if self.rotations > position && position != 0 {
                    self.full_rotations += 1;
                }
                (position + DIAL_SIZE - self.rotations) % DIAL_SIZE
            }
        }
    }
}

impl FromStr for Rotation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rotations = s[1..].parse::<u64>().unwrap();
        let actual_rotations = rotations % DIAL_SIZE;
        let full_rotations = rotations / DIAL_SIZE;
        let direction = if s.starts_with("R") { Direction::Up } else { Direction::Down };
        Ok(Rotation { direction, rotations: actual_rotations, full_rotations })
    }
}

pub fn day1_part1() -> u64 {
    let file = File::open("input.txt").unwrap();
    let file = BufReader::new(file);
    let mut curr_position = 50;
    let mut count = 0;
    for line in file.lines() {
        let line = line.unwrap();
        let mut rotation = line.parse::<Rotation>().unwrap();
        curr_position = rotation.rotate(curr_position);
        if curr_position == 0 {
            count += 1;
        }
    }

    count
}

pub fn day1_part2() -> u64 {
    let file = File::open("input.txt").unwrap();
    let file = BufReader::new(file);
    let mut curr_position = 50;
    let mut count = 0;
    for line in file.lines() {
        let line = line.unwrap();
        let mut rotation = line.parse::<Rotation>().unwrap();
        curr_position = rotation.rotate(curr_position);
        if curr_position == 0 {
            count += 1;
        }
        count += rotation.full_rotations;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = day1_part1();
        assert_eq!(result, 1180);
    }

    #[test]
    fn part2() {
        let result = day1_part2();
        assert_eq!(result, 6892);
    }
}
