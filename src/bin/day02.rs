use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    let input: Vec<&str> = data.lines().collect();

    println!("Part One: {}", part1(&input));

    println!("Part Two: {}", part2(&input));
}

fn part1(input: &[&str]) -> i32 {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let (mut total_double, mut total_triple) = (0, 0);
    for value in input.iter() {
        let (mut twice, mut thrice) = (false, false);
        for letter in alphabet.chars() {
            let count = value.matches(letter).count();
            if count == 2 {
                twice = true;
            }
            if count == 3 {
                thrice = true;
            }
        }
        if twice {
            total_double += 1;
        }
        if thrice {
            total_triple += 1;
        }
    }
    total_double * total_triple
}

fn part2(input: &[&str]) -> String {
    let (mut label1, mut label2) = (String::new(), String::new());
    for x in input.iter() {
        for y in input.iter() {
            let mut differences = 0;
            for (a, b) in x.chars().zip(y.chars()) {
                if a != b {
                    differences += 1;
                }
            }
            if differences == 1 {
                label1 = x.to_string();
                label2 = y.to_string();
                break;
            }
        }
    }
    let mut result = String::new();
    label1.chars().zip(label2.chars()).for_each(|(l1, l2)| {
        if l1 == l2 {
            result.push(l1);
        }
    });
    result
}

#[test]
fn test_part1() {
    let example = [
        "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
    ];
    let result = part1(&example);
    assert_eq!(result, 12);
}

#[test]
fn test_part2() {
    let example = [
        "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
    ];
    let result = part2(&example);
    assert_eq!(result, "fgij");
}
