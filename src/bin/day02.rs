use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");

    let labels: Vec<&str> = data.split_whitespace().collect();

    part_one(&labels);
    part_two(&labels);
}

fn part_one(labels: &[&str]) {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    let mut total_twice = 0;
    let mut total_thrice = 0;

    for item in labels {
        let mut twice = false;
        let mut thrice = false;
        for letter in alphabet.chars() {
            let count = item.matches(letter).count();
            if count == 2 {
                twice = true;
            }
            if count == 3 {
                thrice = true;
            }
        }
        if twice {
            total_twice += 1;
        }
        if thrice {
            total_thrice += 1;
        }
    }
    println!(
        "Twice: {}, Thrice: {}, Checksum: {}",
        total_twice,
        total_thrice,
        total_twice * total_thrice
    );
}

fn part_two(labels: &[&str]) {
    let mut found = false;
    let (mut correct_box1, mut correct_box2) = ("", "");

    for a in labels {
        for b in labels {
            let mut differences = 0;
            for value in a.chars().zip(b.chars()) {
                let (val1, val2) = value;
                if val1 != val2 {
                    differences += 1
                }
            }
            if differences == 1 {
                found = true;
                correct_box1 = a;
                correct_box2 = b;
                break;
            }
        }
        if found {
            break;
        }
    }

    println!("Box 1: {}", correct_box1);

    println!("Box 2: {}", correct_box2);
}
