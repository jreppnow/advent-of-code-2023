use std::io::stdin;

fn main() {
    let mut total: usize = 0;
    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        let mut current = 0;
        for char in line.chars() {
            if let Some(value) = char.to_digit(10) {
                current += value * 10;
                break;
            }
        }

        for char in line.chars().rev() {
            if let Some(value) = char.to_digit(10) {
                current += value;
                break;
            }
        }

        println!("Read \"{line}\", adding {current}");
        total += current as usize;
    }
    println!("Total value: {total}");
}
