#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

struct Ship {
    crates: Vec<Vec<char>>,
}

impl Ship {
    fn number_of_stacks(line: &str) -> usize {
        line.chars().count() / 4
    }

    fn new(input: &str) -> Ship {
        let mut crates: Vec<Vec<char>> =
            Vec::with_capacity(Ship::number_of_stacks(input.lines().next().unwrap()));
        for line in input.lines().rev().skip(1) {
            (0..Ship::number_of_stacks(line)).for_each(|n| {
                let x = line.chars().nth(n * 4 + 1).unwrap();
                if !x.is_ascii_whitespace() {
                    crates[n].push(x);
                }
            });
        }
        Ship { crates }
    }
}

pub fn part_1(string: &str) -> i32 {
    let mut iter = string.split("\n\n");
    let ship = Ship::new(iter.next().unwrap());
    let steps = iter.next().unwrap();
    println!("{:?}", ship.crates);
    todo!()
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(INPUT), 2);
}

pub fn part_2(string: &str) -> i32 {
    todo!()
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(INPUT), 4);
}
