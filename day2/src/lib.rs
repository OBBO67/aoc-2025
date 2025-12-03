use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn invalid(num: &u64) -> bool {
    let num = num.to_string();
    let mid = num.len() / 2;
    let (left, right) = num.split_at(mid);
    left.eq(right)
}

// SLOW
fn invalid_part2(num: &u64) -> bool {
    let num = num.to_string();
    let max_chunk = num.len();
    for chunk_size in 1..max_chunk {
        let chunks: Vec<String> = num
            .chars()
            .collect::<Vec<char>>()
            .chunks(chunk_size)
            .map(|chunk| chunk.iter().collect())
            .collect();

        let invalid = chunks.iter().all(|chunk| chunk.eq(chunks.first().unwrap()));
        if invalid {
            return true;
        }
    }

    false
}

pub fn part_one() -> u64 {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    let mut total = 0;
    for range in f.split(b',') {
        let range = String::from_utf8(range.unwrap()).unwrap();
        let (lower, upper) = range.split_once('-').unwrap();
        let lower = lower.parse::<u64>().unwrap();
        let upper = upper.parse::<u64>().unwrap();
        for num in lower..upper + 1 {
            if invalid(&num) {
                total += num;
            }
        }
    }

    total
}

pub fn part_two() -> u64 {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    let mut total = 0;
    for range in f.split(b',') {
        let range = String::from_utf8(range.unwrap()).unwrap();
        let (lower, upper) = range.split_once('-').unwrap();
        let lower = lower.parse::<u64>().unwrap();
        let upper = upper.parse::<u64>().unwrap();
        for num in lower..upper + 1 {
            if invalid_part2(&num) {
                total += num;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_part1() {
        let result = part_one();
        assert_eq!(result, 8576933996);
    }

    #[test]
    fn day2_part2() {
        let result = part_two();
        assert_eq!(result, 25663320831);
    }
}
