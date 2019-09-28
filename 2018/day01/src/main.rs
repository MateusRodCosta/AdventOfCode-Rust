use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");

    let iter = data.split_whitespace();

    let mut numbers: Vec<i32> = Vec::new();

    iter.for_each(|f| {
        let num = f.parse::<i32>().unwrap();
        numbers.push(num);
    });

    first_part(&numbers);
    second_part(&numbers);
}

fn first_part(numbers: &Vec<i32>) {
    let mut value = 0;

    for num in numbers {
        value += num;
    }
    println!("Result: {}", value);
}

fn second_part(numbers: &Vec<i32>) {
    let mut value: i32 = 0;

    let mut vec: Vec<i32> = Vec::new();
    vec.push(value);

    let mut found = false;

    while !found {
        for num in numbers {
            value += num;
            if vec.contains(&value) {
                found = true;
                break;
            };
            vec.push(value);
        }
    }
    println!("Result: {}", value);
}
