use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    let input: Vec<i32> = data.lines().map(|l| l.parse::<i32>().unwrap()).collect();

    println!("Part One: {}", part1(&input));

    let part2 = part2(&input);

    println!("Part Two: {}", part2.expect("Couldn't get value"));
}

fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

fn part2(input: &[i32]) -> Option<i32> {
    let mut previous = vec![0];
    let mut accumulator = 0;
    for n in input.iter().cycle() {
        accumulator += n;
        if previous.contains(&accumulator) {
            return Some(accumulator);
        }
        previous.push(accumulator);
    }
    None
}

#[test]
fn test_part1_example1() {
    let example = [1, -2, 3, 1];
    let result = part1(&example);
    assert_eq!(result, 3);
}

#[test]
fn test_part1_example2() {
    let example = [1, 1, 1];
    let result = part1(&example);
    assert_eq!(result, 3);
}
#[test]
fn test_part1_example3() {
    let example = [1, 1, -2];
    let result = part1(&example);
    assert_eq!(result, 0);
}
#[test]
fn test_part1_example4() {
    let example = [-1, -2, -3];
    let result = part1(&example);
    assert_eq!(result, -6);
}

#[test]
fn test_part2_example1() {
    let example = [1, -2, 3, 1];
    let result = part2(&example);
    assert_eq!(result, Some(2));
}

#[test]
fn test_part2_example2() {
    let example = [1, -1];
    let result = part2(&example);
    assert_eq!(result, Some(0));
}

#[test]
fn test_part2_example3() {
    let example = [3, 3, 4, -2, -4];
    let result = part2(&example);
    assert_eq!(result, Some(10));
}

#[test]
fn test_part2_example4() {
    let example = [-6, 3, 8, 5, -6];
    let result = part2(&example);
    assert_eq!(result, Some(5));
}

#[test]
fn test_part2_example5() {
    let example = [7, 7, -2, -7, -4];
    let result = part2(&example);
    assert_eq!(result, Some(14));
}
