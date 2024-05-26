use std::io::{self, Write};

fn main() {
    print!("Please enter some input: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read the input");

    let roman: String = input.trim().to_owned();
    let mut result: i32 = 0;
    let mut prev_value: i32 = 0;
    fn get_roman_value(current_roman: char) -> i32 {
        match current_roman {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("Invalid roman numeral")
        }
    }
    for curr_roman in roman.chars().rev() {
        let current_value = get_roman_value(curr_roman);
        if current_value < prev_value {
            result -= current_value
        } else {
            result += current_value
        }
        prev_value = current_value;
    }
    println!("{}", result);
}
