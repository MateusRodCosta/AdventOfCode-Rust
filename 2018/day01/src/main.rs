use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    let array = data.split_whitespace();
    let mut value = 0;
    array.for_each(|f| {
        let val = f.parse::<i32>().unwrap();
        value += val;
    });
    println!("Result: {}", value);
}
