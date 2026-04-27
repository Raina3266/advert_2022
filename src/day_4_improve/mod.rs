#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
pub const INPUT: &str = include_str!("./input.txt");

struct ElfIds {
    start: i32,
    end: i32,
}

impl ElfIds {
    fn new_group(input: &str) -> (Self, Self) {
        let mut group = input.split(',').map(|elf| {
            let mut id = elf.split('-').map(|id| id.parse::<i32>().unwrap());
            (id.next().unwrap(), id.next().unwrap())
        });
        let first_elf = group.next().unwrap();
        let second_elf = group.next().unwrap();
        (
            Self {
                start: first_elf.0,
                end: first_elf.1,
            },
            Self {
                start: second_elf.0,
                end: second_elf.1,
            },
        )
    }
    fn is_contain_mutual(&self, other: ElfIds) -> bool {
        self.start <= other.start && self.end >= other.end
            || other.start <= self.start && other.end >= self.end
    }
    fn is_overlap(&self, other: ElfIds) -> bool {
        self.start < other.end && self.end >= other.start
            || other.start < self.end && self.start <= other.end
            || self.start == other.start
            || self.end == other.end
    }
}

pub fn part_1(string: &str) -> i32 {
    let mut count = 0;
    for pair in string.lines() {
        let (elf1, elf2) = ElfIds::new_group(pair);

        if elf1.is_contain_mutual(elf2) {
            count += 1;
        }
    }
    count
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(INPUT), 2);
}

pub fn part_2(string: &str) -> i32 {
    let mut count = 0;
    for pair in string.lines() {
        let (elf1, elf2) = ElfIds::new_group(pair);

        if elf1.is_overlap(elf2) {
            count += 1;
        }
    }
    count
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(INPUT), 825);
}
