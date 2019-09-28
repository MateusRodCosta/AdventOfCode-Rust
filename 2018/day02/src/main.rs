use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");

    let boxes: Vec<&str> = data.split_whitespace().collect();

    part_one(&boxes);
}

fn part_one(boxes: &Vec<&str>) {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    let mut total_twice = 0;
    let mut total_thrice = 0;

    for item in boxes {
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
